import { createSignal } from "solid-js";
import { API_BASE } from "../api";

export const Auth = () => {
  const [secret, setSecret] = createSignal("");

  const login = (e: Event) => {
    e.preventDefault();

    return fetch(`${API_BASE}/auth/login`, {
      method: "POST",
      body: JSON.stringify({ secret: secret() }),
      headers: { "Content-Type": "application/json" },
    })
      .then((r) => r.json())
      .then((d) => {
        if (d.ok) location.href = "/admin/mailboxes";
      })
      .catch((err) => console.warn(err));
  };

  const logout = (e: Event) => {
    e.preventDefault();

    return fetch(`${API_BASE}/auth/logout`, {
      method: "POST",
      body: "{}",
      headers: { "Content-Type": "application/json" },
    }).then(
      () => setSecret(""),
      (err) => console.warn(err),
    );
  };

  return (
    <main>
      <h1>Auth</h1>

      <form style={{ "margin-block": "2lh" }} onSubmit={(e) => login(e)}>
        <p>
          <input
            type="password"
            size="40"
            placeholder="enter access key"
            value={secret()}
            on:input={(e) => setSecret(e.target.value)}
            style={{ "text-align": "center" }}
          />
        </p>
        <p>
          <input type="submit" value="Authenticate" />
        </p>
      </form>

      <form style={{ "margin-block": "2lh" }}>
        <p>
          <input type="submit" value="Log out" onClick={(e) => logout(e)} />
        </p>
      </form>
    </main>
  );
};
