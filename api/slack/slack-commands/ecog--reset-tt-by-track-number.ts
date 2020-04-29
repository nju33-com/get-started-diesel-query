import { NowRequest, NowResponse } from '@now/node'
import fetch from 'node-fetch'

export default async (
  request: NowRequest,
  response: NowResponse
): Promise<void> => {
  const {
    text,
    channel_id: channelId,
    trigger_id: triggerId
  }: Record<'text' | 'channel_id' | 'trigger_id', string> = request.body

  // @geek_jsasaki か #p_ecogreen
  if (!['D59SRU091', 'C8UNVKYFK'].includes(channelId)) {
    response.status(200).json({
      response_type: 'ephemeral',
      text: '⛔ 実行許可の無いチャンネルです'
    })

    return
  }

  const trackNumber = Number(text)

  if (isNaN(trackNumber)) {
    response.status(200).json({
      response_type: 'ephemeral',
      text: `🤕 テキストには数値のみを渡してください: 得たテキスト \`${text}\``
    })

    return
  }

  try {
    const data: {
      ok: boolean
      error?: string
      response_metadata?: unknown
    } = await (
      await fetch('https://slack.com/api/dialog.open', {
        method: 'POST',
        headers: {
          'content-type': 'application/json',
          authorization:
            'Bearer xoxp-2659222021-179415996097-372606790839-611595f547fae0cd6f34be2666e0e333'
        },
        body: JSON.stringify({
          dialog: {
            callback_id: 'confirm ecog--reset-tt-by-track-number',
            title: `情報を削除します（確認）`,
            submit_label: '削除',
            state: text,
            elements: [
              {
                label: 'もう1度入力',
                name: 'trackNumber',
                type: 'text',
                subtype: 'number',
                placeholder: text
              }
            ]
          },
          trigger_id: triggerId
        })
      })
    ).json()

    if (!data.ok) {
      response.status(200).json({
        response_type: 'ephemeral',
        // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
        text: `🚨 ${data.error}`
      })

      return
    }

    response.status(200).end()
  } catch (err) {
    response.status(200).json({
      response_type: 'ephemeral',
      // eslint-disable-next-line @typescript-eslint/restrict-template-expressions
      text: `🚨 ${err}`
    })
  }
}
