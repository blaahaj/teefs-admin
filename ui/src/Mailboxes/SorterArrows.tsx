import { createSignal } from "solid-js";
import type { TMailbox } from "../dbTypes";

const SORTERS = [
  {
    name: "e-mail address",
    field: "email",
    comparator: (a: TMailbox, b: TMailbox) =>
      a.local_part.localeCompare(b.local_part) ||
      a.domain.localeCompare(b.domain),
  },
  {
    name: "comment",
    field: "comment",
    comparator: (a: TMailbox, b: TMailbox) =>
      (a.comment ?? "").localeCompare(b.comment ?? "") ||
      a.local_part.localeCompare(b.local_part) ||
      a.domain.localeCompare(b.domain),
  },
  {
    name: "created",
    field: "created",
    comparator: (a: TMailbox, b: TMailbox) =>
      a.created.localeCompare(b.created) ||
      a.local_part.localeCompare(b.local_part) ||
      a.domain.localeCompare(b.domain),
  },
] as const;

export type Sorter = (typeof SORTERS)[number];

export const buildSorter = () => {
  const [sorter, setSorter] = createSignal<(typeof SORTERS)[number]>(
    SORTERS.at(-1)!,
  );
  const [reverse, setReverse] = createSignal(false);

  const SorterArrows = (props: {
    readonly field: (typeof SORTERS)[number]["field"];
  }) => (
    <span class="arrowPair">
      <span
        class={`arrow up ${sorter().field === props.field && !reverse() ? "active" : "inactive"}`}
        onClick={() => {
          setSorter(SORTERS.find((s) => s.field === props.field)!);
          setReverse(false);
        }}
      >
        ↑
      </span>
      &#32;
      <span
        class={`arrow up ${sorter().field === props.field && reverse() ? "active" : "inactive"}`}
        onClick={() => {
          setSorter(SORTERS.find((s) => s.field === props.field)!);
          setReverse(true);
        }}
      >
        ↓
      </span>
    </span>
  );

  return {
    sorter,
    reverse,
    SorterArrows,
  };
};
