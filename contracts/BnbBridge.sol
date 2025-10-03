// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

contract BnbSolanaBridge is ReentrancyGuard, Ownable {
    
    struct Validator {
        bool isActive;
        uint256 addedAt;
    }
    
    struct LockEvent {
        address token;
        uint256 amount;
        bytes32 solanaRecipient;
        uint256 timestamp;
        bool processed;
    }
    
    mapping(address => Validator) public validators;
    mapping(bytes32 => LockEvent) public locks;
    mapping(bytes32 => mapping(address => bool)) public validatorSignatures;
    mapping(bytes32 => bool) public processedUnlocks;
    
    address[] public validatorList;
    uint256 public validatorThreshold;
    
    event TokensLocked(
        bytes32 indexed lockId,
        address indexed token,
        address indexed sender,
        uint256 amount,
        bytes32 solanaRecipient
    );
    
    event TokensUnlocked(
        bytes32 indexed unlockId,
        address indexed token,
        address indexed recipient,
        uint256 amount
    );
    
    event ValidatorAdded(address indexed validator);
    event ValidatorRemoved(address indexed validator);
    
    constructor(uint256 _validatorThreshold) Ownable(msg.sender) {
        require(_validatorThreshold > 0, "Threshold must be > 0");
        validatorThreshold = _validatorThreshold;
    }
    
    function lockTokens(
        address token,
        uint256 amount,
        bytes32 solanaRecipient
    ) external nonReentrant returns (bytes32) {
        require(amount > 0, "Amount must be > 0");
        require(token != address(0), "Invalid token");
        require(solanaRecipient != bytes32(0), "Invalid recipient");
        
        IERC20(token).transferFrom(msg.sender, address(this), amount);
        
        bytes32 lockId = keccak256(
            abi.encodePacked(
                token,
                msg.sender,
                amount,
                solanaRecipient,
                block.timestamp,
                block.number
            )
        );
        
        locks[lockId] = LockEvent({
            token: token,
            amount: amount,
            solanaRecipient: solanaRecipient,
            timestamp: block.timestamp,
            processed: false
        });
        
        emit TokensLocked(lockId, token, msg.sender, amount, solanaRecipient);
        
        return lockId;
    }
    
    function unlockTokens(
        bytes32 unlockId,
        address token,
        address recipient,
        uint256 amount,
        bytes[] calldata signatures
    ) external nonReentrant {
        require(!processedUnlocks[unlockId], "Already processed");
        require(signatures.length >= validatorThreshold, "Insufficient signatures");
        require(amount > 0, "Invalid amount");
        
        bytes32 message = keccak256(
            abi.encodePacked(unlockId, token, recipient, amount)
        );
        
        uint256 validSignatures = 0;
        address[] memory signers = new address[](signatures.length);
        
        for (uint256 i = 0; i < signatures.length; i++) {
            address signer = recoverSigner(message, signatures[i]);
            
            if (validators[signer].isActive && !hasSigned(signers, signer, i)) {
                signers[i] = signer;
                validSignatures++;
            }
        }
        
        require(validSignatures >= validatorThreshold, "Invalid signatures");
        
        processedUnlocks[unlockId] = true;
        IERC20(token).transfer(recipient, amount);
        
        emit TokensUnlocked(unlockId, token, recipient, amount);
    }
    
    function addValidator(address validator) external onlyOwner {
        require(validator != address(0), "Invalid validator");
        require(!validators[validator].isActive, "Already validator");
        
        validators[validator] = Validator({
            isActive: true,
            addedAt: block.timestamp
        });
        
        validatorList.push(validator);
        
        emit ValidatorAdded(validator);
    }
    
    function removeValidator(address validator) external onlyOwner {
        require(validators[validator].isActive, "Not a validator");
        
        validators[validator].isActive = false;
        
        emit ValidatorRemoved(validator);
    }
    
    function setValidatorThreshold(uint256 newThreshold) external onlyOwner {
        require(newThreshold > 0, "Threshold must be > 0");
        require(newThreshold <= validatorList.length, "Threshold too high");
        validatorThreshold = newThreshold;
    }
    
    function recoverSigner(
        bytes32 message,
        bytes memory signature
    ) internal pure returns (address) {
        require(signature.length == 65, "Invalid signature length");
        
        bytes32 r;
        bytes32 s;
        uint8 v;
        
        assembly {
            r := mload(add(signature, 32))
            s := mload(add(signature, 64))
            v := byte(0, mload(add(signature, 96)))
        }
        
        if (v < 27) {
            v += 27;
        }
        
        require(v == 27 || v == 28, "Invalid signature v value");
        
        return ecrecover(
            keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", message)),
            v,
            r,
            s
        );
    }
    
    function hasSigned(
        address[] memory signers,
        address signer,
        uint256 upTo
    ) internal pure returns (bool) {
        for (uint256 i = 0; i < upTo; i++) {
            if (signers[i] == signer) {
                return true;
            }
        }
        return false;
    }
    
    function getValidatorCount() external view returns (uint256) {
        return validatorList.length;
    }
    
    function isValidator(address addr) external view returns (bool) {
        return validators[addr].isActive;
    }
}
