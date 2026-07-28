import { type Accessor } from "solid-js";
import type { TMailbox } from "../dbTypes";

export const MailboxRow = ({ item }: { item: Accessor<TMailbox> }) => {
  return (
    <tr>
      <td class="local_part">
        <span class="value">{item().local_part}</span>
      </td>
      <td class="domain">@{item().domain}</td>
      <td class="comment">
        <a href={`/admin/mailboxes/${item().id}`}>
          {item().comment || "<no comment>"}
        </a>
      </td>
      <td class="routes_to">
        {item().routes_to.replace("@maildir.localdomain", "")}
      </td>
      <td>{item().created.substring(0, 10)}</td>
      <td>{item().deleted?.substring(0, 10)}</td>
    </tr>
  );
};
