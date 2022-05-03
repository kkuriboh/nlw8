import { ChatTeardropDots } from 'phosphor-react'
import { Popover } from '@headlessui/react'

export const Widget: React.FC = () => {
	return (
		<Popover className="absolute bottom-5 right-5">
			<Popover.Panel>
				<p>hello world</p>
			</Popover.Panel>
			<Popover.Button className="h-12 bg-brand-500 text-white px-3 rounded-full flex items-center group">
				<ChatTeardropDots className="w-6 h-6" />
				<span className="max-w-0 overflow-hidden group-hover:max-w-xs transition-all duration-500 ease-linear">
					<span className="pl-2"></span>
					feedback
				</span>
			</Popover.Button>
		</Popover>
	)
}
