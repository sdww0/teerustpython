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
// under the License..

#include <thread>
#include <stdio.h>
using namespace std;

#include "app.h"
#include "Enclave_u.h"

void data_producer(void)
{
    sgx_status_t ret = SGX_ERROR_UNEXPECTED;
    ret = ecall_producer(global_eid);
    if (ret != SGX_SUCCESS)
        abort();
}

void data_consumer(void)
{
    sgx_status_t ret = SGX_ERROR_UNEXPECTED;
    ret = ecall_consumer(global_eid);
    if (ret != SGX_SUCCESS)
        abort();
}

void data_init(void)
{
    sgx_status_t ret = SGX_ERROR_UNEXPECTED;
    ret = ecall_initialize(global_eid);
    if (ret != SGX_SUCCESS)
        abort();
}

void data_uninit(void)
{
    sgx_status_t ret = SGX_ERROR_UNEXPECTED;
    ret = ecall_uninitialize(global_eid);
    if (ret != SGX_SUCCESS)
        abort();
}

/* ecall_thread_functions:
 *   Invokes thread functions including mutex, condition variable, etc.
 */
void create_new_thread(void)
{
    
    
}


