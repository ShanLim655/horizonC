use super::*;

impl Launcher {
    pub(crate) fn asking_disclaimer(&self) -> bool {
        (!self.settings.disclaimer_accepted || self.disclaimer_open)
            && !self.asking_consent()
            && self.opening.is_none()
    }

    pub(crate) fn disclaimer_screen(&mut self, cx: &mut Context<Self>) -> Stateful<Div> {
        let t = self.theme;
        let checked = self.disclaimer_checked;
        let acknowledgement = button(
            "disclaimer-acknowledge",
            if checked {
                "✓  I understand and agree"
            } else {
                "□  I understand and agree"
            },
            Kind::Secondary,
            &t,
        )
        .on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
            this.disclaimer_checked = !this.disclaimer_checked;
            cx.notify();
        }));
        let mut accept = button("disclaimer-accept", "Accept & Continue", Kind::Primary, &t);
        if checked {
            accept = accept.on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.settings.disclaimer_accepted = true;
                this.settings.save();
                this.disclaimer_open = false;
                this.disclaimer_checked = false;
                cx.notify();
            }));
        } else {
            accept = disabled(accept);
        }
        let card = plate(&t)
            .flex()
            .flex_col()
            .gap(px(16.))
            .w(px(640.))
            .max_w_full()
            .p(px(28.))
            .bg(t.page)
            .border_1()
            .border_color(t.rule)
            .child(
                text(theme::HEADING)
                    .text_color(t.ink)
                    .child("Before You Play"),
            )
            .child(
                div()
                    .text_size(px(theme::UI.px))
                    .text_color(t.ink_soft)
                    .child("Horizon C is an unofficial, open-source project and is not affiliated with or endorsed by Mojang or Microsoft. We encourage you to purchase an official copy of Minecraft: Java Edition to support its developers. Horizon C does not provide a Minecraft license; use game files from a legally obtained copy."),
            )
            .child(
                div()
                    .text_size(px(theme::UI.px))
                    .text_color(t.ink_soft)
                    .child("Servers that require authenticated Minecraft accounts generally will not accept offline profiles. Offline-mode servers and LAN play may work depending on their configuration."),
            )
            .child(
                div()
                    .text_size(px(theme::UI.px))
                    .text_color(t.ink_soft)
                    .child("By selecting Accept & Continue, you confirm that you understand these limitations and wish to continue."),
            )
            .child(acknowledgement)
            .child(
                div()
                    .flex()
                    .justify_end()
                    .gap(px(10.))
                    .child(
                        button("disclaimer-exit", "Exit", Kind::Ghost, &t)
                            .on_click(|_, _, cx| cx.quit()),
                    )
                    .child(accept),
            );
        div()
            .id("offline-disclaimer")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .px(px(24.))
            .py(px(24.))
            .bg(t.page.opacity(0.96))
            .occlude()
            .child(card)
    }
}
