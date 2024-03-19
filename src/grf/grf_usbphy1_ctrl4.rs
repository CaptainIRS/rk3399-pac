#[doc = "Register `GRF_usbphy1_ctrl4` reader"]
pub type R = crate::R<GrfUsbphy1Ctrl4Spec>;
#[doc = "Register `GRF_usbphy1_ctrl4` writer"]
pub type W = crate::W<GrfUsbphy1Ctrl4Spec>;
#[doc = "Field `USBPHY_CTRL4` reader - usbphy_ctrl4\n\nBit64~79 of usbphy_ctrl register"]
pub type UsbphyCtrl4R = crate::FieldReader<u16>;
#[doc = "Field `USBPHY_CTRL4` writer - usbphy_ctrl4\n\nBit64~79 of usbphy_ctrl register"]
pub type UsbphyCtrl4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - usbphy_ctrl4\n\nBit64~79 of usbphy_ctrl register"]
    #[inline(always)]
    pub fn usbphy_ctrl4(&self) -> UsbphyCtrl4R {
        UsbphyCtrl4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - usbphy_ctrl4\n\nBit64~79 of usbphy_ctrl register"]
    #[inline(always)]
    #[must_use]
    pub fn usbphy_ctrl4(&mut self) -> UsbphyCtrl4W<GrfUsbphy1Ctrl4Spec> {
        UsbphyCtrl4W::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsbphy1Ctrl4Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "usbphy1_ctrl4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usbphy1_ctrl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usbphy1_ctrl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsbphy1Ctrl4Spec;
impl crate::RegisterSpec for GrfUsbphy1Ctrl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usbphy1_ctrl4::R`](R) reader structure"]
impl crate::Readable for GrfUsbphy1Ctrl4Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usbphy1_ctrl4::W`](W) writer structure"]
impl crate::Writable for GrfUsbphy1Ctrl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_usbphy1_ctrl4 to value 0x5554"]
impl crate::Resettable for GrfUsbphy1Ctrl4Spec {
    const RESET_VALUE: u32 = 0x5554;
}
