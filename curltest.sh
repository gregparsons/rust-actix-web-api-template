TESTHOST="127.0.0.1:8080"
PASSWORD="topsecret"
# curl -d "{\"password\":\"$PASSWORD\"}" -H "Content-Type: application/json" http://$TESTHOST/login -v
# curl -d "{\"password\":\"$PASSWORD\"}" -H "Content-Type: application/json" http://$TESTHOST -v

# basic auth for the auth() function which generates the bearer header back to the client
curl --user name:topsecret 127.0.0.1:8080/auth -v

# authenticate through the middleware using the bearer certificate corresponding to the hard-coded password
curl -H "authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoiMSJ9.nNpbmWCiz6-exAOkLdl3nQrzh5p-QEhZ3ko18T8vvII" http://127.0.0.1:8080/secret -v
