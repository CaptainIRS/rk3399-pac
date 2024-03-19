#[doc = "Register `FC_INVACT_2D_1` reader"]
pub type R = crate::R<FcInvact2d1Spec>;
#[doc = "Register `FC_INVACT_2D_1` writer"]
pub type W = crate::W<FcInvact2d1Spec>;
#[doc = "Field `FC_INVACT_2D_1` reader - 2D Input video vertical active pixel region width.\n\nNumber of 2D video vertical active lines \\[11:8\\]."]
pub type FcInvact2d1R = crate::FieldReader;
#[doc = "Field `FC_INVACT_2D_1` writer - 2D Input video vertical active pixel region width.\n\nNumber of 2D video vertical active lines \\[11:8\\]."]
pub type FcInvact2d1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 2D Input video vertical active pixel region width.\n\nNumber of 2D video vertical active lines \\[11:8\\]."]
    #[inline(always)]
    pub fn fc_invact_2d_1(&self) -> FcInvact2d1R {
        FcInvact2d1R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - 2D Input video vertical active pixel region width.\n\nNumber of 2D video vertical active lines \\[11:8\\]."]
    #[inline(always)]
    #[must_use]
    pub fn fc_invact_2d_1(&mut self) -> FcInvact2d1W<FcInvact2d1Spec> {
        FcInvact2d1W::new(self, 0)
    }
}
#[doc = "Frame Composer Input Video VActive pixels Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invact_2d_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invact_2d_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInvact2d1Spec;
impl crate::RegisterSpec for FcInvact2d1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_invact_2d_1::R`](R) reader structure"]
impl crate::Readable for FcInvact2d1Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_invact_2d_1::W`](W) writer structure"]
impl crate::Writable for FcInvact2d1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INVACT_2D_1 to value 0"]
impl crate::Resettable for FcInvact2d1Spec {
    const RESET_VALUE: u8 = 0;
}
