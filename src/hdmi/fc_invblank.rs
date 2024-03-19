#[doc = "Register `FC_INVBLANK` reader"]
pub type R = crate::R<FcInvblankSpec>;
#[doc = "Register `FC_INVBLANK` writer"]
pub type W = crate::W<FcInvblankSpec>;
#[doc = "Field `V_IN_BLANK` reader - Input video Vertical blanking pixel region width.\n\nNumber of Vertical blanking lines \\[0...255\\]."]
pub type VInBlankR = crate::FieldReader;
#[doc = "Field `V_IN_BLANK` writer - Input video Vertical blanking pixel region width.\n\nNumber of Vertical blanking lines \\[0...255\\]."]
pub type VInBlankW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Input video Vertical blanking pixel region width.\n\nNumber of Vertical blanking lines \\[0...255\\]."]
    #[inline(always)]
    pub fn v_in_blank(&self) -> VInBlankR {
        VInBlankR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input video Vertical blanking pixel region width.\n\nNumber of Vertical blanking lines \\[0...255\\]."]
    #[inline(always)]
    #[must_use]
    pub fn v_in_blank(&mut self) -> VInBlankW<FcInvblankSpec> {
        VInBlankW::new(self, 0)
    }
}
#[doc = "Frame Composer Input Video VBlank Pixels Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fc_invblank::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc_invblank::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcInvblankSpec;
impl crate::RegisterSpec for FcInvblankSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fc_invblank::R`](R) reader structure"]
impl crate::Readable for FcInvblankSpec {}
#[doc = "`write(|w| ..)` method takes [`fc_invblank::W`](W) writer structure"]
impl crate::Writable for FcInvblankSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FC_INVBLANK to value 0"]
impl crate::Resettable for FcInvblankSpec {
    const RESET_VALUE: u8 = 0;
}
