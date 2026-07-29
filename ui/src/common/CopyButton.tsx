import { createSignal, type Accessor } from "solid-js";
import { CheckIcon } from "./CheckIcon";
import { CopyIcon } from "./CopyIcon";

import "./CopyButton.css";

export const CopyButton = ({ text }: { text: Accessor<string> }) => {
  const [animating, setAnimating] = createSignal(false);

  return (
    <span
      class={`copy ${animating() ? "animating" : ""}`}
      onClick={() =>
        navigator.clipboard.writeText(text()).then(() => {
          setAnimating(true);
          setTimeout(() => setAnimating(false), 2000);
        })
      }
    >
      {animating() ?
        <CheckIcon />
      : <CopyIcon />}
    </span>
  );
};
