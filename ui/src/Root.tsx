import "./Root.css";

export const Root = ({ children }: { children?: any }) => (
  <>
    <div class="banner">Teefs Admin</div>
    <div>{children}</div>
  </>
);
