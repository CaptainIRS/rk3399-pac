#[doc = "Register `HDCPREG_ANCONF` reader"]
pub type R = crate::R<HdcpregAnconfSpec>;
#[doc = "Register `HDCPREG_ANCONF` writer"]
pub type W = crate::W<HdcpregAnconfSpec>;
#[doc = "Field `OANBYPASS` reader - When oanbypass=1, the value of AN used in the HDCP engine comes from the hdcpreg_an0 to hdcpreg_an7 registers. When oanbypass=0, the value of AN used in the HDCP engine comes from the random number input."]
pub type OanbypassR = crate::BitReader;
#[doc = "Field `OANBYPASS` writer - When oanbypass=1, the value of AN used in the HDCP engine comes from the hdcpreg_an0 to hdcpreg_an7 registers. When oanbypass=0, the value of AN used in the HDCP engine comes from the random number input."]
pub type OanbypassW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When oanbypass=1, the value of AN used in the HDCP engine comes from the hdcpreg_an0 to hdcpreg_an7 registers. When oanbypass=0, the value of AN used in the HDCP engine comes from the random number input."]
    #[inline(always)]
    pub fn oanbypass(&self) -> OanbypassR {
        OanbypassR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When oanbypass=1, the value of AN used in the HDCP engine comes from the hdcpreg_an0 to hdcpreg_an7 registers. When oanbypass=0, the value of AN used in the HDCP engine comes from the random number input."]
    #[inline(always)]
    #[must_use]
    pub fn oanbypass(&mut self) -> OanbypassW<HdcpregAnconfSpec> {
        OanbypassW::new(self, 0)
    }
}
#[doc = "When oanbypass=1, the value of AN used in the HDCP engine comes from the hdcpreg_an0 to hdcpreg_an7 registers. When oanbypass=0, the value of AN used in the HDCP engine comes from the random number input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcpreg_anconf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcpreg_anconf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpregAnconfSpec;
impl crate::RegisterSpec for HdcpregAnconfSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcpreg_anconf::R`](R) reader structure"]
impl crate::Readable for HdcpregAnconfSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcpreg_anconf::W`](W) writer structure"]
impl crate::Writable for HdcpregAnconfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCPREG_ANCONF to value 0"]
impl crate::Resettable for HdcpregAnconfSpec {
    const RESET_VALUE: u8 = 0;
}
