#[doc = "Register `EMMCCORE_ARG` reader"]
pub type R = crate::R<EmmccoreArgSpec>;
#[doc = "Register `EMMCCORE_ARG` writer"]
pub type W = crate::W<EmmccoreArgSpec>;
#[doc = "Field `COMMANDARGUMENT1` reader - The SD Command Argument is specified as bit39-8 of Command- Format."]
pub type Commandargument1R = crate::FieldReader<u32>;
#[doc = "Field `COMMANDARGUMENT1` writer - The SD Command Argument is specified as bit39-8 of Command- Format."]
pub type Commandargument1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The SD Command Argument is specified as bit39-8 of Command- Format."]
    #[inline(always)]
    pub fn commandargument1(&self) -> Commandargument1R {
        Commandargument1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The SD Command Argument is specified as bit39-8 of Command- Format."]
    #[inline(always)]
    #[must_use]
    pub fn commandargument1(&mut self) -> Commandargument1W<EmmccoreArgSpec> {
        Commandargument1W::new(self, 0)
    }
}
#[doc = "Argument register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_arg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_arg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreArgSpec;
impl crate::RegisterSpec for EmmccoreArgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emmccore_arg::R`](R) reader structure"]
impl crate::Readable for EmmccoreArgSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_arg::W`](W) writer structure"]
impl crate::Writable for EmmccoreArgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_ARG to value 0"]
impl crate::Resettable for EmmccoreArgSpec {
    const RESET_VALUE: u32 = 0;
}
