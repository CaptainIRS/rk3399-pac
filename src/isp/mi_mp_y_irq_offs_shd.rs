#[doc = "Register `MI_MP_Y_IRQ_OFFS_SHD` reader"]
pub type R = crate::R<MiMpYIrqOffsShdSpec>;
#[doc = "Field `mp_y_irq_offs` reader - Reaching this offset value by the current offset\n\ncounter for addressing main picture Y component, JPEG or\n\nraw data leads to generation of fill level interrupt fill_mp_y."]
pub type MpYIrqOffsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:28 - Reaching this offset value by the current offset\n\ncounter for addressing main picture Y component, JPEG or\n\nraw data leads to generation of fill level interrupt fill_mp_y."]
    #[inline(always)]
    pub fn mp_y_irq_offs(&self) -> MpYIrqOffsR {
        MpYIrqOffsR::new((self.bits >> 3) & 0x03ff_ffff)
    }
}
#[doc = "Shadow register of fill level interrupt offset value for main \n\n\n\npicture Y component, JPEG or raw data\n\nNote: This register protects from non-aligned access. Refer to MI_MP_Y_BASE_AD_INIT \n\n\n\nregister description for details. \n\n\n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_mp_y_irq_offs_shd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiMpYIrqOffsShdSpec;
impl crate::RegisterSpec for MiMpYIrqOffsShdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_mp_y_irq_offs_shd::R`](R) reader structure"]
impl crate::Readable for MiMpYIrqOffsShdSpec {}
#[doc = "`reset()` method sets MI_MP_Y_IRQ_OFFS_SHD to value 0"]
impl crate::Resettable for MiMpYIrqOffsShdSpec {
    const RESET_VALUE: u32 = 0;
}
