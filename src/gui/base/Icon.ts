import m, {Children, Component, Vnode} from "mithril"
import {theme} from "../theme"
import type {lazy} from "@tutao/tutanota-utils"
import {assertMainOrNode} from "../../api/common/Env"
import {BootIcons, BootIconsSvg} from "./icons/BootIcons";
import {Icons} from "./icons/Icons";

assertMainOrNode()

export type AllIcons = BootIcons | Icons

export type IconAttrs = {
	icon: AllIcons
	class?: string
	large?: boolean
	style?: Record<string, any>
	container?: "span" | "div" // defaults to "span"
}

export type lazyIcon = lazy<AllIcons>

let IconsSvg = {}

import("./icons/Icons.js").then(IconsModule => {
	IconsSvg = IconsModule.IconsSvg
})

export class Icon implements Component<IconAttrs> {
	view(vnode: Vnode<IconAttrs>): Children {
		const icon = BootIconsSvg[vnode.attrs.icon as any] ? BootIconsSvg[vnode.attrs.icon as any] : IconsSvg[vnode.attrs.icon as any]
		const container = vnode.attrs.container || "span"
		return m(
				container + ".icon",
				{
					"aria-hidden": "true",
					class: this.getClass(vnode.attrs),
					style: this.getStyle(vnode.attrs.style),
				},
				m.trust(icon),
		) // icon is typed, so we may not embed untrusted data
	}

	getStyle(
			style: Record<string, any> | null,
	): {
		fill: string
	} {
		style = style ? style : {}

		if (!style.fill) {
			style.fill = theme.content_accent
		}

		return style as { fill: string }
	}

	getClass(attrs: IconAttrs): string {
		if (attrs.large) {
			return "icon-large"
		} else if (attrs.class) {
			return attrs.class
		} else {
			return ""
		}
	}
}

export function progressIcon(): Vnode<IconAttrs> {
	return m(Icon, {
		icon: BootIcons.Progress,
		class: "icon-large icon-progress",
	})
}