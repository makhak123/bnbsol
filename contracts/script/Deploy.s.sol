// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "forge-std/Script.sol";
import "../BnbBridge.sol";

contract DeployScript is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        
        vm.startBroadcast(deployerPrivateKey);
        
        // Deploy with 2/3 validator threshold
        BnbSolanaBridge bridge = new BnbSolanaBridge(2);
        
        console.log("BnbSolanaBridge deployed at:", address(bridge));
        
        vm.stopBroadcast();
    }
}
