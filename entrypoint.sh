#!/bin/bash

if [ ! -f /app/Rocket.toml ]; then
    echo "[release]
address = \"0.0.0.0\"
port = 9999
secret_key = \"$(LC_ALL=C tr -dc 'A-Za-z0-9' </dev/urandom | head -c 88)\"

[default.limits]
forms = \"64 kB\"
json = \"1 MiB\"
msgpack = \"2 MiB\"
\"file/jpg\" = \"5 MiB\"" > /app/Rocket.toml
fi

# Replace env vars in JavaScript files
echo "Replacing env vars in JS"
for file in /app/static/js/*.*.js;
do
  echo "Processing $file ...";

  # Use the existing JS file as template
  if [ ! -f $file.tmpl.js ]; then
    cp $file $file.tmpl.js
  fi

  envsubst '$VUE_APP_ROOT_API,$VUE_APP_SENTRY_DSN' < $file.tmpl.js > $file
  rm $file.tmpl.js
done
echo "Finished processing all JS files"

# Run download service
/app/wgt_backend