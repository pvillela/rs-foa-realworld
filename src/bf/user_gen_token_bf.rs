pub type UserGenTokenBfT = func(user model.User) (string, error)

pub type UserGenTokenEcdsaBfCfgSrc = func() (
	private_key ecdsa.PrivateKey,
	token_time_to_live time.Duration,
)

pub fn user_gen_token_ecdsa_bf_c(
cfg_src: UserGenTokenEcdsaBfCfgSrc,
) -> UserGenTokenBfT {
	private_key, token_time_to_live := cfg_src()
	return user_gen_token_bf_c[ecdsa.private_key](private_key, token_time_to_live, jwt.signing_method_e_s256)
}

pub struct UserGenTokenHmacBfCfgInfo {
pub  key: &{814 <nil> byte},
pub  token_time_to_live: &{time Duration},
}


pub type UserGenTokenHmacBfCfgSrc = func() UserGenTokenHmacBfCfgInfo

pub fn user_gen_token_hmac_bf_c(
cfg_src: UserGenTokenHmacBfCfgSrc,
) -> UserGenTokenBfT {
	info := cfg_src()
	return user_gen_token_bf_c[[]byte](info.key, info.token_time_to_live, jwt.signing_method_h_s256)
}

fn user_gen_token_bf_c(
key: K,
token_time_to_live: &{time Duration},
signing_method: &{jwt SigningMethod},
) -> UserGenTokenBfT {
	return func(user model.User) (string, error) {
		if user.username == "" {
			return "", errx.new_errx(nil, "can't generate token for empty user")
		}

		claims := jwt.registered_claims{
			subject:	user.username,
			expires_at:	jwt.new_numeric_date(time.now().add(token_time_to_live)),
			issuer:		"real-world-demo-backend",
		}

		jws, err := jwt.new_with_claims(signing_method, &claims).signed_string(key)
		if err != nil {
			return "", errx.errx_of(err)
		}

		return jws, nil
	}
}

