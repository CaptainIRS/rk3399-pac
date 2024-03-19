#[doc = "Register `IF_TYPE` reader"]
pub type R = crate::R<IfTypeSpec>;
#[doc = "Register `IF_TYPE` writer"]
pub type W = crate::W<IfTypeSpec>;
#[doc = "Field `IF_TYPE` reader - InfoFrame Packet Type Code. It can be set as (0x80 + \n\nInfoFrame Type Code) and send any type of infoframe \n\ndefined in CEA-861C. \n\nCommonly, we set it as 0x83(0x80 + 0x03, 0x03 is the type \n\ncode of SPD InfoFrame) and send SPD infoframe."]
pub type IfTypeR = crate::FieldReader;
#[doc = "Field `IF_TYPE` writer - InfoFrame Packet Type Code. It can be set as (0x80 + \n\nInfoFrame Type Code) and send any type of infoframe \n\ndefined in CEA-861C. \n\nCommonly, we set it as 0x83(0x80 + 0x03, 0x03 is the type \n\ncode of SPD InfoFrame) and send SPD infoframe."]
pub type IfTypeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - InfoFrame Packet Type Code. It can be set as (0x80 + \n\nInfoFrame Type Code) and send any type of infoframe \n\ndefined in CEA-861C. \n\nCommonly, we set it as 0x83(0x80 + 0x03, 0x03 is the type \n\ncode of SPD InfoFrame) and send SPD infoframe."]
    #[inline(always)]
    pub fn if_type(&self) -> IfTypeR {
        IfTypeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - InfoFrame Packet Type Code. It can be set as (0x80 + \n\nInfoFrame Type Code) and send any type of infoframe \n\ndefined in CEA-861C. \n\nCommonly, we set it as 0x83(0x80 + 0x03, 0x03 is the type \n\ncode of SPD InfoFrame) and send SPD infoframe."]
    #[inline(always)]
    #[must_use]
    pub fn if_type(&mut self) -> IfTypeW<IfTypeSpec> {
        IfTypeW::new(self, 0)
    }
}
#[doc = "InfoFrame Packet Type Code.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`if_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`if_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfTypeSpec;
impl crate::RegisterSpec for IfTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_type::R`](R) reader structure"]
impl crate::Readable for IfTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`if_type::W`](W) writer structure"]
impl crate::Writable for IfTypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IF_TYPE to value 0"]
impl crate::Resettable for IfTypeSpec {
    const RESET_VALUE: u32 = 0;
}
