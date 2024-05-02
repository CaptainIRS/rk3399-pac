#[doc = "Register `MI_BYTE_CNT` reader"]
pub type R = crate::R<MiByteCntSpec>;
#[doc = "Field `byte_cnt` reader - Counter value specifies the number of JPEG or RAW\n\ndata bytes of the last transmitted frame. Updated at frame\n\nend. A soft reset will set the byte counter to zero.\n\n"]
pub type ByteCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - Counter value specifies the number of JPEG or RAW\n\ndata bytes of the last transmitted frame. Updated at frame\n\nend. A soft reset will set the byte counter to zero.\n\n"]
    #[inline(always)]
    pub fn byte_cnt(&self) -> ByteCntR {
        ByteCntR::new(self.bits & 0x0fff_ffff)
    }
}
#[doc = "Counter value of JPEG or RAW data bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_byte_cnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiByteCntSpec;
impl crate::RegisterSpec for MiByteCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_byte_cnt::R`](R) reader structure"]
impl crate::Readable for MiByteCntSpec {}
#[doc = "`reset()` method sets MI_BYTE_CNT to value 0"]
impl crate::Resettable for MiByteCntSpec {
    const RESET_VALUE: u32 = 0;
}
