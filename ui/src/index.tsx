/* @refresh reload */
import { render } from "solid-js/web";
import "./index.css";
import { Route, Router } from "@solidjs/router";
import { Root } from "./Root";
import { Home } from "./Home";
import { Mailbox } from "./Mailbox";
import { AddMailbox } from "./AddMailbox/inbox";
import { Mailboxes } from "./Mailboxes";
import { Auth } from "./Auth";

const root = document.getElementById("root");

render(
  () => (
    <Router root={Root}>
      <Route path="/admin" component={Home} />
      <Route path="/admin/auth" component={Auth} />
      <Route path="/admin/mailboxes" component={Mailboxes} />
      <Route path="/admin/mailboxes/add" component={AddMailbox} />
      <Route path="/admin/mailboxes/:id" component={Mailbox} />
    </Router>
  ),
  root!,
);
