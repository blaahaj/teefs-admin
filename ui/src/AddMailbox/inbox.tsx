import { createResource, createSignal } from "solid-js";

import "./inbox.css";
import { API_BASE } from "../api";

type local_partResponse = {
  readonly six_hex: string;
  readonly four_dotted_words: string;
};

export const AddMailbox = () => {
  const [local_partSuggestions, local_partSuggestionsActions] = createResource<
    local_partResponse | undefined
  >(() =>
    fetch(`${API_BASE}/random-local-parts`)
      .then((r) => r.json())
      .then((d: local_partResponse) => d),
  );

  const [style, setStyle] = createSignal(
    "four_dotted_words" as keyof local_partResponse,
  );

  const [domain, setDomain] = createSignal("teefs.eu");
  const [comment, setComment] = createSignal("");
  const [routes_to, set_routes_to] = createSignal(
    "teefs/main@maildir.localdomain",
  );

  return (
    <main>
      <h1>Add mailbox</h1>

      <br />

      <div class="form">
        <div class="kv">
          <div>
            Local part:
            <br />
            <button on:click={() => local_partSuggestionsActions.refetch()}>
              🔄
            </button>
          </div>
          <div>
            <div>
              <input
                type="radio"
                checked={style() === "four_dotted_words"}
                on:change={() => setStyle("four_dotted_words")}
              />{" "}
              {local_partSuggestions()?.four_dotted_words}
            </div>
            <div>
              <input
                type="radio"
                checked={style() === "six_hex"}
                on:change={() => setStyle("six_hex")}
              />{" "}
              {local_partSuggestions()?.six_hex}
            </div>
          </div>
        </div>

        <div class="kv">
          <div>Domain:</div>
          <div>
            <input
              type="text"
              value={domain()}
              on:input={(e) => setDomain(e.target.value)}
            />
          </div>
        </div>

        <div class="kv">
          <div>Comment:</div>
          <div>
            <input
              type="text"
              autofocus
              value={comment()}
              placeholder="Say what this mailbox is for"
              on:input={(e) => setComment(e.target.value)}
            />
          </div>
        </div>

        <div class="kv">
          <div>Routes to:</div>
          <div>
            <input
              type="text"
              value={routes_to()}
              on:input={(e) => set_routes_to(e.target.value)}
            />
          </div>
        </div>
      </div>

      <button
        on:click={() => {
          fetch(`${API_BASE}/mailboxes`, {
            method: "POST",
            credentials: "include",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify({
              mailbox: {
                local_part: local_partSuggestions()![style()],
                domain: domain(),
                comment: comment(),
                routes_to: routes_to(),
              },
            }),
          }).then(async (r) => {
            if (r.status === 200) {
              const id = (await r.json()).mailbox.id;
              navigation.navigate(`/admin/mailboxes/${id}`);
            } else {
              console.error(r);
            }
          });
        }}
      >
        Add mailbox
      </button>

      <br />
      <br />
      <br />

      <div>
        <a href="/admin/mailboxes">Mailboxes</a>
      </div>
    </main>
  );
};
