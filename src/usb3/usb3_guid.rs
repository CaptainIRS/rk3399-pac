#[doc = "Register `USB3_GUID` reader"]
pub type R = crate::R<Usb3GuidSpec>;
#[doc = "Register `USB3_GUID` writer"]
pub type W = crate::W<Usb3GuidSpec>;
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
    pub fn userid(&mut self) -> UseridW<Usb3GuidSpec> {
        UseridW::new(self, 0)
    }
}
#[doc = "Global User ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_guid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_guid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3GuidSpec;
impl crate::RegisterSpec for Usb3GuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_guid::R`](R) reader structure"]
impl crate::Readable for Usb3GuidSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_guid::W`](W) writer structure"]
impl crate::Writable for Usb3GuidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_GUID to value 0x1234_5678"]
impl crate::Resettable for Usb3GuidSpec {
    const RESET_VALUE: u32 = 0x1234_5678;
}
