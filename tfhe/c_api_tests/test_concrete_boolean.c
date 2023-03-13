#include <tfhe.h>
#include <stdio.h>
#include <inttypes.h>

#define GOTO_IF_TRUE(label, cond) if ((cond) == 1) {\
  goto label;\
  }

int ex1(const ConcreteClientKey *client_key) {
    int ok;
    FheBool *lhs = NULL;
    FheBool *rhs = NULL;
    FheBool *result = NULL;

    bool lhs_clear = 0;
    bool rhs_clear = 1;

    ok = fhe_bool_try_encrypt_with_client_key(lhs_clear, client_key, &lhs);
    GOTO_IF_TRUE(exit, ok != 0)

    ok = fhe_bool_try_encrypt_with_client_key(rhs_clear, client_key, &rhs);
    GOTO_IF_TRUE(exit, ok != 0)

    ok = fhe_bool_bitand(lhs, rhs, &result);
    GOTO_IF_TRUE(exit, ok != 0)

    bool clear;
    ok = fhe_bool_decrypt(result, client_key, &clear);
    GOTO_IF_TRUE(exit, ok != 0)

    printf("Got %d, Expected %d\n", (int)clear, (int) lhs_clear & rhs_clear);

exit:
    fhe_bool_destroy(lhs);
    fhe_bool_destroy(rhs);
    fhe_bool_destroy(result);

    return ok;
}

int ex2(const ConcreteClientKey *client_key) {
    int ok;
    FheUint8 *lhs = NULL;
    FheUint8 *rhs = NULL;
    FheUint8 *result = NULL;

    uint8_t lhs_clear = 123;
    uint8_t rhs_clear = 14;

    ok = fhe_uint8_try_encrypt_with_client_key(lhs_clear, client_key, &lhs);
    GOTO_IF_TRUE(exit, ok != 0)

    ok = fhe_uint8_try_encrypt_with_client_key(rhs_clear, client_key, &rhs);
    GOTO_IF_TRUE(exit, ok != 0)

    ok = fhe_uint8_add(lhs, rhs, &result);
    GOTO_IF_TRUE(exit, ok != 0)

    uint8_t clear;
    ok = fhe_uint8_decrypt(result, client_key, &clear);
    GOTO_IF_TRUE(exit, ok != 0)

    printf("Got %" PRIu8 " Expected %" PRIu8 "\n", clear, lhs_clear + rhs_clear);
    
 exit:
    fhe_uint8_destroy(lhs);
    fhe_uint8_destroy(rhs);
    fhe_uint8_destroy(result);
  return ok;
}

int ex3(const ConcreteClientKey *client_key, const ConcretePublicKey* public_key) {
    int ok;
    FheUint8 *lhs = NULL;
    FheUint8 *rhs = NULL;
    FheUint8 *result = NULL;

    uint8_t lhs_clear = 123;
    uint8_t rhs_clear = 14;

    ok = fhe_uint8_try_encrypt_with_client_key(lhs_clear, client_key, &lhs);
    GOTO_IF_TRUE(exit, ok != 0)

    ok = fhe_uint8_try_encrypt_with_client_key(rhs_clear, client_key, &rhs);
    GOTO_IF_TRUE(exit, ok != 0)

    ok = fhe_uint8_sub(lhs, rhs, &result);
    GOTO_IF_TRUE(exit, ok != 0)

    uint8_t clear;
    ok = fhe_uint8_decrypt(result, client_key, &clear);
    GOTO_IF_TRUE(exit, ok != 0)

    printf("Got %" PRIu8 " Expected %" PRIu8 "\n", clear, lhs_clear - rhs_clear);

exit:
    fhe_uint8_destroy(lhs);
    fhe_uint8_destroy(rhs);
    fhe_uint8_destroy(result);
    return ok;
}


int main(void)
{
  
  ConfigBuilder *builder;
  Config *config;
  config_builder_all_disabled(&builder);
  config_builder_enable_default_bool(&builder);
  config_builder_enable_default_uint8(&builder);


  config_builder_build(builder, &config);

  ConcreteClientKey *client_key = NULL;
  ConcreteServerKey *server_key = NULL;
  ConcretePublicKey *public_key = NULL;

  generate_keys(config, &client_key, &server_key);
  public_key_new(client_key, &public_key);

  set_server_key(server_key);

  ex1(client_key);
  ex2(client_key);
  ex3(client_key, public_key);
  
  concrete_client_key_destroy(client_key);
  concrete_public_key_destroy(public_key);
  concrete_server_key_destroy(server_key);

  return EXIT_SUCCESS;
}