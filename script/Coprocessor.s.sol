// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import "../src/Coprocessor.sol";

contract MyScript is Script {
    function run(address chain_fusion_canister_address) external returns(address){
        // the private key of the deployer is the first private key printed by running anvil
        uint256 deployerPrivateKey = 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80;

        // we use that key to broadcast all following transactions
        vm.startBroadcast(deployerPrivateKey);

        Coprocessor coprocessor = new Coprocessor();
        coprocessor.updateCoprocessor(chain_fusion_canister_address);
        // this creates the contract. it will have the same address every time if we use a
        // new instance of anvil for every deployment.

        // we create 3 jobs
        for (uint256 index = 0; index < 3; index++) {
            coprocessor.newJob{value: 0.1 ether}();
        }

        vm.stopBroadcast();
        return address(coprocessor);
    }
}