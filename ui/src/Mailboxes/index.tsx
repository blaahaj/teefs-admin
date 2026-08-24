import { createResource, createSignal } from "solid-js";
import "./index.css";
import { MailboxesTable } from "./MailboxesTable";
import type { TMailbox } from "../dbTypes";
import { API_BASE } from "../api";

export function Mailboxes() {
  const [mailboxes, _mailboxActions] = createResource<readonly TMailbox[]>(() =>
    fetch(`${API_BASE}/mailboxes`).then((r) => {
      if (r.status === 403) location.href = "/admin/auth";
      else if (r.status === 200) {
        return r.json().then((d) => d.mailboxes);
      }
    }),
  );

  const [filterText, setFilterText] = createSignal("");

  return (
    <>
      <h1>Mailboxes</h1>

      <div class="tools">
        <div>
          <button on:click={() => navigation.navigate("/admin/mailboxes/add")}>
            Add...
          </button>
        </div>
        <div>
          {" "}
          <input
            type="text"
            placeholder="Quick filter"
            value={filterText()}
            on:input={(e) => setFilterText(e.target.value)}
          />
        </div>
      </div>

      <p style={{ "margin-block": "1em" }}></p>

      <MailboxesTable mailboxes={mailboxes} filterText={filterText} />

      <section id="spacer"></section>
    </>
  );
}
