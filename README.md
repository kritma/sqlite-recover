# sqlite-recover

* Uses native sqlite methods to recover malformed database
* Build with rust

## install
```sh
npm install sqlite-recover
```

## usage
```ts
import sr from 'sqlite-recover'

const controller = new AbortController()

await sr.recoverSqlAsync("./malformed.db", "./recovered.db", (sql_err) => {
    console.log(sql_err.message)
}, controller.signal);

```

## api
```ts
// recovers database by using native methods (cant report recovery errors yet)
function recover(path: string, recovered: string): string | null
function recoverAsync(path: string, recovered: string, signal: AbortSignal): Promise<undefined>

// recovers database by executing sql
export function recoverSql(path: string, recovered: string, callback: (err: Error) => void): string | null
export function recoverSqlAsync(path: string, recovered: string, callback: (err: Error) => void, signal: AbortSignal): Promise<undefined>

```
