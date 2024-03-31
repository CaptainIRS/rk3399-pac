#[doc = "Register `USB20_HOST1_CON1` reader"]
pub type R = crate::R<Usb20Host1Con1Spec>;
#[doc = "Register `USB20_HOST1_CON1` writer"]
pub type W = crate::W<Usb20Host1Con1Spec>;
#[doc = "Field `FLADJ_VAL` reader - fladj_val\n\nMust set this register to 0x20."]
pub type FladjValR = crate::FieldReader;
#[doc = "Field `FLADJ_VAL` writer - fladj_val\n\nMust set this register to 0x20."]
pub type FladjValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FLADJ_VAL_COMMON` reader - fladj_val_common\n\nMust set this register to 0x20."]
pub type FladjValCommonR = crate::FieldReader;
#[doc = "Field `FLADJ_VAL_COMMON` writer - fladj_val_common\n\nMust set this register to 0x20."]
pub type FladjValCommonW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - fladj_val\n\nMust set this register to 0x20."]
    #[inline(always)]
    pub fn fladj_val(&self) -> FladjValR {
        FladjValR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - fladj_val_common\n\nMust set this register to 0x20."]
    #[inline(always)]
    pub fn fladj_val_common(&self) -> FladjValCommonR {
        FladjValCommonR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - fladj_val\n\nMust set this register to 0x20."]
    #[inline(always)]
    #[must_use]
    pub fn fladj_val(&mut self) -> FladjValW<Usb20Host1Con1Spec> {
        FladjValW::new(self, 0)
    }
    #[doc = "Bits 6:11 - fladj_val_common\n\nMust set this register to 0x20."]
    #[inline(always)]
    #[must_use]
    pub fn fladj_val_common(&mut self) -> FladjValCommonW<Usb20Host1Con1Spec> {
        FladjValCommonW::new(self, 6)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Usb20Host1Con1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "USB20 Host1 GRF register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb20_host1_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb20_host1_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb20Host1Con1Spec;
impl crate::RegisterSpec for Usb20Host1Con1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb20_host1_con1::R`](R) reader structure"]
impl crate::Readable for Usb20Host1Con1Spec {}
#[doc = "`write(|w| ..)` method takes [`usb20_host1_con1::W`](W) writer structure"]
impl crate::Writable for Usb20Host1Con1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB20_HOST1_CON1 to value 0x0820"]
impl crate::Resettable for Usb20Host1Con1Spec {
    const RESET_VALUE: u32 = 0x0820;
}
