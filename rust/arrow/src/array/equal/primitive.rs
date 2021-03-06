// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::mem::size_of;

use crate::array::ArrayData;

use super::utils::equal_len;

pub(super) fn primitive_equal<T>(
    lhs: &ArrayData,
    rhs: &ArrayData,
    lhs_start: usize,
    rhs_start: usize,
    len: usize,
) -> bool {
    let byte_width = size_of::<T>();
    let lhs_values = &lhs.buffers()[0].data()[lhs.offset() * byte_width..];
    let rhs_values = &rhs.buffers()[0].data()[rhs.offset() * byte_width..];

    if lhs.null_count() == 0 && rhs.null_count() == 0 {
        // without nulls, we just need to compare slices
        equal_len(
            lhs_values,
            rhs_values,
            lhs_start * byte_width,
            rhs_start * byte_width,
            len * byte_width,
        )
    } else {
        // with nulls, we need to compare item by item whenever it is not null
        (0..len).all(|i| {
            let lhs_pos = lhs_start + i;
            let rhs_pos = rhs_start + i;
            let lhs_is_null = lhs.is_null(lhs_pos);
            let rhs_is_null = rhs.is_null(rhs_pos);

            lhs_is_null
                || (lhs_is_null == rhs_is_null)
                    && equal_len(
                        lhs_values,
                        rhs_values,
                        lhs_pos * byte_width,
                        rhs_pos * byte_width,
                        byte_width, // 1 * byte_width since we are comparing a single entry
                    )
        })
    }
}
