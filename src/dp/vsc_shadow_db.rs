#[doc = "Register `VSC_SHADOW_DB%s` reader"]
pub type R = crate::R<VscShadowDbSpec>;
#[doc = "Register `VSC_SHADOW_DB%s` writer"]
pub type W = crate::W<VscShadowDbSpec>;
#[doc = "Field `VSC_SHADOW_DB0_VSC_SHADOW_DB7` reader - VSC shadow data bytes 0 ~ 7"]
pub type VscShadowDb0VscShadowDb7R = crate::FieldReader;
#[doc = "Field `VSC_SHADOW_DB0_VSC_SHADOW_DB7` writer - VSC shadow data bytes 0 ~ 7"]
pub type VscShadowDb0VscShadowDb7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VSC shadow data bytes 0 ~ 7"]
    #[inline(always)]
    pub fn vsc_shadow_db0_vsc_shadow_db7(&self) -> VscShadowDb0VscShadowDb7R {
        VscShadowDb0VscShadowDb7R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VSC shadow data bytes 0 ~ 7"]
    #[inline(always)]
    #[must_use]
    pub fn vsc_shadow_db0_vsc_shadow_db7(&mut self) -> VscShadowDb0VscShadowDb7W<VscShadowDbSpec> {
        VscShadowDb0VscShadowDb7W::new(self, 0)
    }
}
#[doc = "VSC shadow data bytes 0 ~ 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vsc_shadow_db::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vsc_shadow_db::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VscShadowDbSpec;
impl crate::RegisterSpec for VscShadowDbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vsc_shadow_db::R`](R) reader structure"]
impl crate::Readable for VscShadowDbSpec {}
#[doc = "`write(|w| ..)` method takes [`vsc_shadow_db::W`](W) writer structure"]
impl crate::Writable for VscShadowDbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets VSC_SHADOW_DB%s to value 0"]
impl crate::Resettable for VscShadowDbSpec {
    const RESET_VALUE: u32 = 0;
}
