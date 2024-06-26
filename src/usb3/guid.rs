#[doc = "Register `GUID` reader"]
pub type R = crate::R<GuidSpec>;
#[doc = "Register `GUID` writer"]
pub type W = crate::W<GuidSpec>;
#[doc = "Field `USERID` reader - USERID\n\nApplication-programmable ID field."]
pub type UseridR = crate::FieldReader<u32>;
#[doc = "Field `USERID` writer - USERID\n\nApplication-programmable ID field."]
pub type UseridW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USERID\n\nApplication-programmable ID field."]
    #[inline(always)]
    pub fn userid(&self) -> UseridR {
        UseridR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USERID\n\nApplication-programmable ID field."]
    #[inline(always)]
    #[must_use]
    pub fn userid(&mut self) -> UseridW<GuidSpec> {
        UseridW::new(self, 0)
    }
}
#[doc = "Global User ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`guid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`guid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GuidSpec;
impl crate::RegisterSpec for GuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`guid::R`](R) reader structure"]
impl crate::Readable for GuidSpec {}
#[doc = "`write(|w| ..)` method takes [`guid::W`](W) writer structure"]
impl crate::Writable for GuidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUID to value 0x1234_5678"]
impl crate::Resettable for GuidSpec {
    const RESET_VALUE: u32 = 0x1234_5678;
}
