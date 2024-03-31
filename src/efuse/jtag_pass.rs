#[doc = "Register `JTAG_PASS` reader"]
pub type R = crate::R<JtagPassSpec>;
#[doc = "Register `JTAG_PASS` writer"]
pub type W = crate::W<JtagPassSpec>;
#[doc = "Field `JTAG_PASSWD` reader - Jtag password for jtag monitor"]
pub type JtagPasswdR = crate::FieldReader<u32>;
#[doc = "Field `JTAG_PASSWD` writer - Jtag password for jtag monitor"]
pub type JtagPasswdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Jtag password for jtag monitor"]
    #[inline(always)]
    pub fn jtag_passwd(&self) -> JtagPasswdR {
        JtagPasswdR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Jtag password for jtag monitor"]
    #[inline(always)]
    #[must_use]
    pub fn jtag_passwd(&mut self) -> JtagPasswdW<JtagPassSpec> {
        JtagPasswdW::new(self, 0)
    }
}
#[doc = "Jtag password\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_pass::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_pass::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagPassSpec;
impl crate::RegisterSpec for JtagPassSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag_pass::R`](R) reader structure"]
impl crate::Readable for JtagPassSpec {}
#[doc = "`write(|w| ..)` method takes [`jtag_pass::W`](W) writer structure"]
impl crate::Writable for JtagPassSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JTAG_PASS to value 0x0cf7_680a"]
impl crate::Resettable for JtagPassSpec {
    const RESET_VALUE: u32 = 0x0cf7_680a;
}
