use super::{CLIENT_NAME, USER, USER2, TestUser, get_test_url, sessionStorageKey};
use std::collections::HashMap;
use etebase::{Account, Client, User};
use etebase::error::{Error, Result};
use etebase::test_helpers::test_reset;
use etebase::utils::from_base64;
use test_context::TestContext;

pub struct BaseContext {
    pub client: Client
}

impl TestContext for BaseContext {
    fn setup() -> Self {
        etebase::init().expect("failed to initialize etebase");
        let client = Client::new(CLIENT_NAME, &get_test_url())
            .expect("failed to create etebase client");
        Self { client }
    }
}

pub struct CreateUsersContext {
    pub accounts: HashMap<String, Account>,
}

impl CreateUsersContext {
    fn user_reset(user: &TestUser) -> Result<()> {
        let client = Client::new(CLIENT_NAME, &get_test_url())?;
        let body_struct = etebase::test_helpers::SignupBody {
            user: &etebase::User::new(user.username,user.email),
            salt: &from_base64(user.salt)?,
            pubkey: &from_base64(user.pubkey)?,
            login_pubkey: &from_base64(user.loginPubkey)?,
            encrypted_content: &from_base64(user.encryptedContent)?,
        };

        test_reset(&client, body_struct)?;

        Ok(())
    }
}

impl TestContext for CreateUsersContext {
    fn setup() -> Self {
        let BaseContext { client } = BaseContext::setup();

        let mut accounts = HashMap::new();
        for test_user in [USER, USER2].iter() {
            // Attempt to log in. If failed because user does not exist, sign up
            // Otherwise, reset.
            let user = User::new(&test_user.username, &test_user.email);
            let acct = Account::login(client.clone(), &test_user.username, &test_user.password)
                .unwrap_or_else(|err| {
                    if let Error::NotFound(_) = err {
                        Account::signup(client.clone(), &user, &test_user.password)
                            .expect("failed to sign up")
                    } else {
                        panic!("unexpected error: {:?}", err);
                    }
                });
            acct.logout().expect("failed to log out of existing account");
            Self::user_reset(&test_user).expect("failed to reset user");

            let session_key = from_base64(sessionStorageKey)
                .expect("failed to read session storage key from base64");
            let mut ret = Account::restore(client.clone(), test_user.storedSession, Some(&session_key))
                .expect("failed to restore session");
            ret.force_server_url(&get_test_url())
                .expect("failed to force server URL for account");
            ret.fetch_token()
                .expect("failed to fetch auth token");

            accounts.insert(test_user.username.to_owned(), acct);
        }

        Self { accounts }
    }

    fn teardown(self) {

    }
}
