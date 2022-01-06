#[cfg(feature = "my_feature")]
pub fn is_feature_set() -> bool{
	true
}


#[cfg(not(feature = "my_feature"))]
pub fn is_feature_set() -> bool{
	false
}
