#[doc = "Register `FRM_CNT` reader"]
pub type R = crate::R<FrmCntSpec>;
#[doc = "Register `FRM_CNT` writer"]
pub type W = crate::W<FrmCntSpec>;
#[doc = "Field `FRM_CNT` reader - frame counter\n\nSelf increase one after a frame operation is finished. Write arbitrary\n\nvalue to clear to zero."]
pub type FrmCntR = crate::FieldReader<u32>;
#[doc = "Field `FRM_CNT` writer - frame counter\n\nSelf increase one after a frame operation is finished. Write arbitrary\n\nvalue to clear to zero."]
pub type FrmCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - frame counter\n\nSelf increase one after a frame operation is finished. Write arbitrary\n\nvalue to clear to zero."]
    #[inline(always)]
    pub fn frm_cnt(&self) -> FrmCntR {
        FrmCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - frame counter\n\nSelf increase one after a frame operation is finished. Write arbitrary\n\nvalue to clear to zero."]
    #[inline(always)]
    #[must_use]
    pub fn frm_cnt(&mut self) -> FrmCntW<FrmCntSpec> {
        FrmCntW::new(self, 0)
    }
}
#[doc = "frame counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frm_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frm_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrmCntSpec;
impl crate::RegisterSpec for FrmCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frm_cnt::R`](R) reader structure"]
impl crate::Readable for FrmCntSpec {}
#[doc = "`write(|w| ..)` method takes [`frm_cnt::W`](W) writer structure"]
impl crate::Writable for FrmCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRM_CNT to value 0"]
impl crate::Resettable for FrmCntSpec {
    const RESET_VALUE: u32 = 0;
}
