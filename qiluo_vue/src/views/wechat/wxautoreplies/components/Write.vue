<script setup lang="tsx">
import { Form, FormSchema } from '@/components/Form'
import { useForm } from '@/hooks/web/useForm'
import { PropType, reactive, watch, ref } from 'vue'
import { useValidator } from '@/hooks/web/useValidator'
import { useI18n } from '@/hooks/web/useI18n'
import { GetWxAccountsList } from '@/api/wx_accounts'

const { t } = useI18n()

const { required } = useValidator()

const props = defineProps({
  currentRow: {
    type: Object as PropType<any>,
    default: () => null
  }
})

const formSchema = ref<FormSchema[]>([
  {
    field: 'account_id',
    label: t('wx_auto_replies.account_id'),
    component: 'Select',
    componentProps: {
      filterable: true
    },
    optionApi: async () => {
      const res = await GetWxAccountsList({ page_num: 1, page_size: 100 })
      return (res.data.list || []).map((item: any) => ({
        label: item.account_name,
        value: item.id
      }))
    },
    colProps: {
      span: 24
    }
  },
  {
    field: 'reply_type',
    label: t('wx_auto_replies.reply_type'),
    component: 'Select',
    componentProps: {
      options: [
        { label: t('wx_auto_replies.reply_type_subscribe'), value: 1 },
        { label: t('wx_auto_replies.reply_type_keyword'), value: 2 },
        { label: t('wx_auto_replies.reply_type_default'), value: 3 }
      ]
    },
    colProps: {
      span: 24
    }
  },
  {
    field: 'keyword',
    label: t('wx_auto_replies.keyword'),
    component: 'Input',
    componentProps: {
      placeholder: t('wx_auto_replies.keyword')
    },
    colProps: {
      span: 24
    }
  },
  {
    field: 'match_type',
    label: t('wx_auto_replies.match_type'),
    component: 'Select',
    componentProps: {
      options: [
        { label: t('wx_auto_replies.match_type_full'), value: 1 },
        { label: t('wx_auto_replies.match_type_partial'), value: 2 },
        { label: t('wx_auto_replies.match_type_regex'), value: 3 }
      ]
    },
    colProps: {
      span: 24
    }
  },
  {
    field: 'message_type',
    label: t('wx_auto_replies.message_type'),
    component: 'Select',
    componentProps: {
      options: [
        { label: t('wx_auto_replies.message_type_text'), value: 'text' },
        { label: t('wx_auto_replies.message_type_image'), value: 'image' },
        { label: t('wx_auto_replies.message_type_voice'), value: 'voice' },
        { label: t('wx_auto_replies.message_type_music'), value: 'music' },
        { label: t('wx_auto_replies.message_type_news'), value: 'news' },
        { label: t('wx_auto_replies.message_type_video'), value: 'video' }
      ]
    },
    colProps: {
      span: 24
    }
  },
  {
    field: 'content',
    label: t('wx_auto_replies.content'),
    component: 'InputTextarea',
    colProps: {
      span: 24
    }
  },
  {
    field: 'media_id',
    label: t('wx_auto_replies.media_id'),
    component: 'Input',
    componentProps: {
      placeholder: t('wx_auto_replies.media_id')
    },
    colProps: {
      span: 24
    }
  },
  {
    field: 'title',
    label: t('wx_auto_replies.title'),
    component: 'Input',
    colProps: {
      span: 24
    }
  },
  {
    field: 'description',
    label: t('wx_auto_replies.description'),
    component: 'Input',
    colProps: {
      span: 24
    }
  },
  {
    field: 'pic_url',
    label: t('wx_auto_replies.pic_url'),
    component: 'Input',
    colProps: {
      span: 24
    }
  },
  {
    field: 'url',
    label: t('wx_auto_replies.url'),
    component: 'Input',
    colProps: {
      span: 24
    }
  },
  {
    field: 'music_url',
    label: t('wx_auto_replies.music_url'),
    component: 'Input',
    colProps: {
      span: 24
    }
  },
  {
    field: 'hq_music_url',
    label: t('wx_auto_replies.hq_music_url'),
    component: 'Input',
    colProps: {
      span: 24
    }
  },
  {
    field: 'thumb_media_id',
    label: t('wx_auto_replies.thumb_media_id'),
    component: 'Input',
    colProps: {
      span: 24
    }
  },
  {
    field: 'status',
    label: t('wx_auto_replies.status'),
    component: 'Select',
    componentProps: {
      options: [
        { label: t('wx_auto_replies.status_disabled'), value: 0 },
        { label: t('wx_auto_replies.status_enabled'), value: 1 }
      ]
    },
    colProps: {
      span: 24
    }
  },
  {
    field: 'priority',
    label: t('wx_auto_replies.priority'),
    component: 'InputNumber',
    colProps: {
      span: 24
    }
  }
])

const rules = reactive({ account_id: [required()], reply_type: [required()] })

const { formRegister, formMethods } = useForm()
const { setValues, getFormData, getElFormExpose } = formMethods

const submit = async () => {
  const elForm = await getElFormExpose()
  const valid = await elForm?.validate().catch((err) => {
    console.log(err)
  })
  if (valid) {
    const formData = await getFormData()
    return formData
  }
}

watch(
  () => props.currentRow,
  (currentRow) => {
    if (!currentRow) return
    console.log(currentRow)
    setValues(currentRow)
  },
  {
    deep: true,
    immediate: true
  }
)

defineExpose({
  submit
})
</script>

<template>
  <Form :rules="rules" @register="formRegister" :schema="formSchema" />
</template>
