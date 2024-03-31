#[doc = "Register `USRID` reader"]
pub type R = crate::R<UsridSpec>;
#[doc = "Register `USRID` writer"]
pub type W = crate::W<UsridSpec>;
#[doc = "Field `USRID` reader - User identification register.\n\nThe default value is determined by Configuration Value."]
pub type UsridR = crate::FieldReader<u32>;
#[doc = "Field `USRID` writer - User identification register.\n\nThe default value is determined by Configuration Value."]
pub type UsridW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User identification register.\n\nThe default value is determined by Configuration Value."]
    #[inline(always)]
    pub fn usrid(&self) -> UsridR {
        UsridR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User identification register.\n\nThe default value is determined by Configuration Value."]
    #[inline(always)]
    #[must_use]
    pub fn usrid(&mut self) -> UsridW<UsridSpec> {
        UsridW::new(self, 0)
    }
}
#[doc = "User ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usrid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usrid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsridSpec;
impl crate::RegisterSpec for UsridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usrid::R`](R) reader structure"]
impl crate::Readable for UsridSpec {}
#[doc = "`write(|w| ..)` method takes [`usrid::W`](W) writer structure"]
impl crate::Writable for UsridSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USRID to value 0x0796_7797"]
impl crate::Resettable for UsridSpec {
    const RESET_VALUE: u32 = 0x0796_7797;
}
