#[doc = "Register `VSC_SHADOW_PB%s` reader"]
pub type R = crate::R<VscShadowPbSpec>;
#[doc = "Register `VSC_SHADOW_PB%s` writer"]
pub type W = crate::W<VscShadowPbSpec>;
#[doc = "Field `VSC_SHADOW_PB0_VSC_SHADOW_PB1` reader - VSC shadow parity bytes 0 ~ 1"]
pub type VscShadowPb0VscShadowPb1R = crate::FieldReader;
#[doc = "Field `VSC_SHADOW_PB0_VSC_SHADOW_PB1` writer - VSC shadow parity bytes 0 ~ 1"]
pub type VscShadowPb0VscShadowPb1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VSC shadow parity bytes 0 ~ 1"]
    #[inline(always)]
    pub fn vsc_shadow_pb0_vsc_shadow_pb1(&self) -> VscShadowPb0VscShadowPb1R {
        VscShadowPb0VscShadowPb1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VSC shadow parity bytes 0 ~ 1"]
    #[inline(always)]
    #[must_use]
    pub fn vsc_shadow_pb0_vsc_shadow_pb1(&mut self) -> VscShadowPb0VscShadowPb1W<VscShadowPbSpec> {
        VscShadowPb0VscShadowPb1W::new(self, 0)
    }
}
#[doc = "VSC shadow parity byte 0 ~ 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsc_shadow_pb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsc_shadow_pb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VscShadowPbSpec;
impl crate::RegisterSpec for VscShadowPbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsc_shadow_pb::R`](R) reader structure"]
impl crate::Readable for VscShadowPbSpec {}
#[doc = "`write(|w| ..)` method takes [`vsc_shadow_pb::W`](W) writer structure"]
impl crate::Writable for VscShadowPbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets VSC_SHADOW_PB%s to value 0"]
impl crate::Resettable for VscShadowPbSpec {
    const RESET_VALUE: u32 = 0;
}
