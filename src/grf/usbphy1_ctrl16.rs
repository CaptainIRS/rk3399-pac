#[doc = "Register `usbphy1_ctrl16` reader"]
pub type R = crate::R<Usbphy1Ctrl16Spec>;
#[doc = "Register `usbphy1_ctrl16` writer"]
pub type W = crate::W<Usbphy1Ctrl16Spec>;
#[doc = "Field `USBPHY_CTRL16` reader - usbphy_ctrl16\n\nBit256~271 of usbphy_ctrl register"]
pub type UsbphyCtrl16R = crate::FieldReader<u16>;
#[doc = "Field `USBPHY_CTRL16` writer - usbphy_ctrl16\n\nBit256~271 of usbphy_ctrl register"]
pub type UsbphyCtrl16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - usbphy_ctrl16\n\nBit256~271 of usbphy_ctrl register"]
    #[inline(always)]
    pub fn usbphy_ctrl16(&self) -> UsbphyCtrl16R {
        UsbphyCtrl16R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - usbphy_ctrl16\n\nBit256~271 of usbphy_ctrl register"]
    #[inline(always)]
    #[must_use]
    pub fn usbphy_ctrl16(&mut self) -> UsbphyCtrl16W<Usbphy1Ctrl16Spec> {
        UsbphyCtrl16W::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Usbphy1Ctrl16Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "usbphy1_ctrl16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy1_ctrl16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy1_ctrl16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usbphy1Ctrl16Spec;
impl crate::RegisterSpec for Usbphy1Ctrl16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphy1_ctrl16::R`](R) reader structure"]
impl crate::Readable for Usbphy1Ctrl16Spec {}
#[doc = "`write(|w| ..)` method takes [`usbphy1_ctrl16::W`](W) writer structure"]
impl crate::Writable for Usbphy1Ctrl16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usbphy1_ctrl16 to value 0x0200"]
impl crate::Resettable for Usbphy1Ctrl16Spec {
    const RESET_VALUE: u32 = 0x0200;
}
