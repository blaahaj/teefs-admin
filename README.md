# Teefs Admin

A web UI for Teefs administration.

## The API server

```shell
cd api
docker buildx build --tag teefs-admin-api .
```

Running it (assuming the database is in `./db`):

```shell
docker run --rm -it -v $PWD/db:/data -e SQLITE_DB_FILE=/data/teefs.with-id.db -e PORT=8800 -e BIND_ADDRESS=0.0.0.0 --publish 8801:8800 teefs-admin-api:latest
```

Then you can `curl http://<your-en0-address>:8801/api/mailboxes`.

Or, copy the binary out from the image:

```shell
container=$( docker create teefs-admin-api:latest )
docker export $container | tar -xvf- teefs-admin-api
docker rm $container
```

## The UI

```shell
cd ui
pnpm install
pnpm build
```

Then copy `./dist` to wherever it is needed.

## For boop

UNTESTED

```shell
pushd api
rsync -av --exclude=target ./ boop.teefs.eu:teefs-admin-api/
ssh -t boop.teefs.eu 'cd teefs-admin-api && docker build --tag teefs-admin-api .'
cat <<<'
    cd teefs-admin-api
    container=$( docker create teefs-admin-api:latest )
    docker export $container | tar -xvf- teefs-admin-api
    docker rm $container
' | ssh boop.teefs.eu bash
popd

pushd ui
pnpm install
pnpm build
rsync -av --delete dist/ boop.teefs.eu:teefs-admin-api/dist/
popd
```

- `/etc/nginx/teefs-admin-api.service` for the unit file
- `/etc/nginx/teefs-admin-api.env` for the auth secret
- Static files are read from `rachel`'s 'teefs-admin-api/dist/` dir
