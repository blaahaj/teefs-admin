import "./MailboxesTable.css";
import { Index, type Accessor, type Resource } from "solid-js";
import type { TMailbox } from "../dbTypes";
import { buildSorter } from "./SorterArrows";
import { MailboxRow } from "./MailboxRow";

export const MailboxesTable = ({
  mailboxes,
  filterText,
}: {
  mailboxes: Resource<readonly TMailbox[] | undefined>;
  filterText: Accessor<string>;
}) => {
  const sorterStuff = buildSorter();

  const sortedMailboxes = () => {
    const m = mailboxes();
    if (!m) return [];

    let list = [...m].sort(sorterStuff.sorter().comparator);
    if (sorterStuff.reverse()) list.reverse();

    const t = filterText().trim().toLocaleLowerCase() ?? null;
    if (t !== null) {
      list = list.filter(
        (m) =>
          m.local_part.toLocaleLowerCase().includes(t) ||
          m.comment?.toLocaleLowerCase().includes(t),
      );
    }
    return list;
  };

  return (
    <table class="mailboxes">
      <thead>
        <tr>
          <th>
            Local part
            <sorterStuff.SorterArrows field="email" />
          </th>
          <th>Domain</th>
          <th>
            Comment
            <sorterStuff.SorterArrows field="comment" />
          </th>
          <th>Routes to</th>
          <th>
            Created
            <sorterStuff.SorterArrows field="created" />
          </th>
          <th>Deleted</th>
        </tr>
      </thead>
      <tbody>
        <Index each={sortedMailboxes()} fallback={<p>Error</p>}>
          {(item, _index) => <MailboxRow item={item} />}
        </Index>
      </tbody>
    </table>
  );
};
