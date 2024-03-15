#[doc = "Register `FC_INVACT_2D_0` reader"]
pub type R = crate::R<FcInvact2d0Spec>;
#[doc = "Register `FC_INVACT_2D_0` writer"]
pub type W = crate::W<FcInvact2d0Spec>;
#[doc = "Field `FC_INVACT_2D_0` reader - 2D Input video vertical active pixel region width. Number of 2D video vertical active lines \\[7:0\\]."]
pub type FcInvact2d0R = crate::FieldReader;
#[doc = "Field `FC_INVACT_2D_0` writer - 2D Input video vertical active pixel region width. Number of 2D video vertical active lines \\[7:0\\]."]
pub type FcInvact2d0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 2D Input video vertical active pixel region width. Number of 2D video vertical active lines \\[7:0\\]."]
    #[inline(always)]
    pub fn fc_invact_2d_0(&self) -> FcInvact2d0R {
        FcInvact2d0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - 2D Input video vertical active pixel region width. Number of 2D video vertical active lines \\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn fc_invact_2d_0(&mut self) -> FcInvact2d0W<FcInvact2d0Spec> {
        FcInvact2d0W::new(self, 0)
    }
}
#[doc = "2D Input video vertical active pixel region width. Number of 2D video vertical active lines \\[7:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invact_2d_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invact_2d_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInvact2d0Spec;
impl crate::RegisterSpec for FcInvact2d0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_invact_2d_0::R`](R) reader structure"]
impl crate::Readable for FcInvact2d0Spec {}
#[doc = "`write(|w| ..)` method takes [`fc_invact_2d_0::W`](W) writer structure"]
impl crate::Writable for FcInvact2d0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INVACT_2D_0 to value 0"]
impl crate::Resettable for FcInvact2d0Spec {
    const RESET_VALUE: u8 = 0;
}
