#[cfg(test)]
mod tests {
	use crate::password::{hash_password, verify_password};

	#[test]
	fn test_hash_password_success() {
		let password = "test_password";
		let hash_result = hash_password(password);

		assert!(hash_result.is_ok());
		let hash = hash_result.unwrap();
		assert!(!hash.is_empty());
	}

	#[test]
	fn test_verify_password_success() {
		let password = "test_password";
		let hash = hash_password(password).unwrap();

		let verify_result = verify_password(password, &hash);
		assert!(verify_result.is_ok());
		assert!(verify_result.unwrap());
	}

	#[test]
	fn test_verify_password_failure() {
		let password = "test_password";
		let wrong_password = "wrong_password";
		let hash = hash_password(password).unwrap();

		let verify_result = verify_password(wrong_password, &hash);
		assert!(verify_result.is_ok());
		assert!(!verify_result.unwrap());
	}

	#[test]
	fn test_verify_password_invalid_hash() {
		let password = "test_password";
		let invalid_hash = "invalid_hash_string";

		let verify_result = verify_password(password, invalid_hash);
		assert!(verify_result.is_err());
	}
}
