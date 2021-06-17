// Copyright 2018-2021 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]
use ink_lang as ink;
use stub::TemplateStub;
#[ink::contract]
mod erc20 {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
    };
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_env::call::FromAccountId;
    use stub::TemplateStub;

    /// A simple ERC-20 contract.
    #[ink(storage)]
    pub struct Erc20 {
        /// Total token supply.
        total_supply: Lazy<Balance>,
        //合约模板id和hash映射.
        template_index_hash_map:Lazy<StorageHashMap<Hash,u32>>,
    }


    impl Erc20 {
        /// Creates a new ERC-20 contract with the specified initial supply.
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let instance = Self {
                total_supply: Lazy::new(initial_supply),
                template_index_hash_map:Lazy::new(Default::default()),
            };
            instance
        }

        /// 添加合约的id和hash值
        #[ink(message)]
        pub fn add_template_hash(&mut self,hash:Hash,id:u32)->bool{
            let value = self.template_index_hash_map.insert(hash,id);
            if let None = value {
                //如果该key不存在,返回true
                true
            }else{
                false
            }
        }

        /// 查询所有模板的hash值队列
        #[ink(message)]
        pub fn get_all_template_hash(&self){
            self.template_index_hash_map;
        }

        /// Returns the total token supply.
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            *self.total_supply
        }


        #[ink(message)]
        pub fn get_template_id(&self,account_id:AccountId) -> u32 {
            ink_env::debug_message("-------------1");
            let template:TemplateStub = FromAccountId::from_account_id(account_id);
            ink_env::debug_message("-------------2");
            template.get_id()
        }
    }
}
