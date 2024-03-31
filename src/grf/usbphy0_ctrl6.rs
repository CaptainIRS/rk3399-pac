#[doc = "Register `usbphy0_ctrl6` reader"]
pub type R = crate::R<Usbphy0Ctrl6Spec>;
#[doc = "Register `usbphy0_ctrl6` writer"]
pub type W = crate::W<Usbphy0Ctrl6Spec>;
#[doc = "Field `USBPHY_CTRL6` reader - usbphy_ctrl6\n\nBit96~111 of usbphy_ctrl register"]
pub type UsbphyCtrl6R = crate::FieldReader<u16>;
#[doc = "Field `USBPHY_CTRL6` writer - usbphy_ctrl6\n\nBit96~111 of usbphy_ctrl register"]
pub type UsbphyCtrl6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - usbphy_ctrl6\n\nBit96~111 of usbphy_ctrl register"]
    #[inline(always)]
    pub fn usbphy_ctrl6(&self) -> UsbphyCtrl6R {
        UsbphyCtrl6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - usbphy_ctrl6\n\nBit96~111 of usbphy_ctrl register"]
    #[inline(always)]
    #[must_use]
    pub fn usbphy_ctrl6(&mut self) -> UsbphyCtrl6W<Usbphy0Ctrl6Spec> {
        UsbphyCtrl6W::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Usbphy0Ctrl6Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "usbphy0_ctrl6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphy0_ctrl6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy0_ctrl6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usbphy0Ctrl6Spec;
impl crate::RegisterSpec for Usbphy0Ctrl6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphy0_ctrl6::R`](R) reader structure"]
impl crate::Readable for Usbphy0Ctrl6Spec {}
#[doc = "`write(|w| ..)` method takes [`usbphy0_ctrl6::W`](W) writer structure"]
impl crate::Writable for Usbphy0Ctrl6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usbphy0_ctrl6 to value 0x05"]
impl crate::Resettable for Usbphy0Ctrl6Spec {
    const RESET_VALUE: u32 = 0x05;
}
