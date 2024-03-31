#[doc = "Register `ARG` reader"]
pub type R = crate::R<ArgSpec>;
#[doc = "Register `ARG` writer"]
pub type W = crate::W<ArgSpec>;
#[doc = "Field `COMMANDARGUMENT1` reader - The SD Command Argument is specified as bit39-8 of Command-\n\nFormat."]
pub type Commandargument1R = crate::FieldReader<u32>;
#[doc = "Field `COMMANDARGUMENT1` writer - The SD Command Argument is specified as bit39-8 of Command-\n\nFormat."]
pub type Commandargument1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The SD Command Argument is specified as bit39-8 of Command-\n\nFormat."]
    #[inline(always)]
    pub fn commandargument1(&self) -> Commandargument1R {
        Commandargument1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The SD Command Argument is specified as bit39-8 of Command-\n\nFormat."]
    #[inline(always)]
    #[must_use]
    pub fn commandargument1(&mut self) -> Commandargument1W<ArgSpec> {
        Commandargument1W::new(self, 0)
    }
}
#[doc = "Argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArgSpec;
impl crate::RegisterSpec for ArgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arg::R`](R) reader structure"]
impl crate::Readable for ArgSpec {}
#[doc = "`write(|w| ..)` method takes [`arg::W`](W) writer structure"]
impl crate::Writable for ArgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARG to value 0"]
impl crate::Resettable for ArgSpec {
    const RESET_VALUE: u32 = 0;
}
