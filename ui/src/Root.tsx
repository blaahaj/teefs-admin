import "./Root.css";

export const Root = ({ children }: { children?: any }) => {
  return (
    <>
      <div class="banner">
        <div>Teefs Admin</div>
      </div>

      <p>
        <a href="/admin">Home</a>
      </p>

      <div>{children}</div>
    </>
  );
};
