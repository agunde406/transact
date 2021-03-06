/*
 * Copyright 2018 Bitwise IO, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

use crate::transaction::TransactionPair;

/// During processing of the Transaction, something unexpected happened.
/// The `Executor` immediately retries the `TransactionPair` for all of these
/// errors.
#[derive(Debug)]
pub enum ExecutionAdapterError {
    /// Executing the transaction took too much time and so abort
    TimeOutError(TransactionPair),
    /// This ExecutionAdaptor does not have the capability to process the `TransactionPair`
    /// given to it. This can happen due to a timing error in routing the `TransactionPair`
    /// to the `ExecutionAdapter`.
    RoutingError(TransactionPair),
}
