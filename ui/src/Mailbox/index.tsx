import type { Params } from "@solidjs/router";
import { createResource } from "solid-js";
import { isValid } from "ulid";

import { type TMailbox } from "../dbTypes";
import { CopyButton } from "../common/CopyButton";

export const Mailbox = ({ params }: { params: Params }) => {
  const { id } = params;
  if (!id || !isValid(id)) {
    return <p>Error</p>;
  }

  const [mailbox, mailboxActions] = createResource<TMailbox | undefined>(() =>
    fetch(`/admin/api/mailboxes/${id}`)
      .then((r) => r.json())
      .then((d) => d.mailbox),
  );

  const emailAddress = () => {
    const m = mailbox();
    return m ? `${m.local_part}@${m.domain}` : undefined;
  };

  return (
    <main>
      <h1>Mailbox</h1>

      <pre
        style={{
          "text-align": "start",
          "margin-inline": "auto",
          width: "fit-content",
        }}
      >
        {`ID:          ${mailbox()?.id}`}
        <br />
        {`Local part:  ${mailbox()?.local_part}`}
        <br />
        {`Domain:      ${mailbox()?.domain}`}
        <br />
        {`Comment:     ${mailbox()?.comment}`}
        <br />
        {`Routes to:   ${mailbox()?.routes_to}`}
        <br />
        {`Created:     ${mailbox()?.created}`}
        <br />
        {`Deleted:     ${mailbox()?.deleted}`}
      </pre>

      {emailAddress() && (
        <div>
          <span>{emailAddress()}</span>
          <span style={{ "margin-inline-start": "1em" }}>
            <CopyButton text={() => emailAddress()!} />
          </span>
        </div>
      )}

      <br />

      <div>
        <button
          on:click={async () => {
            await fetch(
              `/admin/api/mailboxes/${id}/${mailbox()?.deleted ? "undelete" : "delete"}`,
              { method: "POST" },
            ).then(() => mailboxActions.refetch());
          }}
        >
          {mailbox()?.deleted ? "Undelete" : "Delete"}
        </button>
      </div>

      <br />

      <div>
        <a href="/admin/mailboxes">Mailboxes</a>
      </div>
    </main>
  );
};
