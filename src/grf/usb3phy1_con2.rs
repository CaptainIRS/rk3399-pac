#[doc = "Register `USB3PHY1_CON2` reader"]
pub type R = crate::R<Usb3phy1Con2Spec>;
#[doc = "Register `USB3PHY1_CON2` writer"]
pub type W = crate::W<Usb3phy1Con2Spec>;
#[doc = "Field `VBUS_VOLTAGE` reader - TCPC vbus voltage\n\nTCPC vbus voltage"]
pub type VbusVoltageR = crate::FieldReader<u16>;
#[doc = "Field `VBUS_VOLTAGE` writer - TCPC vbus voltage\n\nTCPC vbus voltage"]
pub type VbusVoltageW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "vbus source overcurrent\n\nValue on reset: 15"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VbusOvercurrentN {
    #[doc = "0: vbus source over current"]
    B0 = 0,
    #[doc = "1: vbus source not over current"]
    B1 = 1,
}
impl From<VbusOvercurrentN> for u8 {
    #[inline(always)]
    fn from(variant: VbusOvercurrentN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VbusOvercurrentN {
    type Ux = u8;
}
#[doc = "Field `VBUS_OVERCURRENT_N` reader - vbus source overcurrent"]
pub type VbusOvercurrentNR = crate::FieldReader<VbusOvercurrentN>;
impl VbusOvercurrentNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VbusOvercurrentN> {
        match self.bits {
            0 => Some(VbusOvercurrentN::B0),
            1 => Some(VbusOvercurrentN::B1),
            _ => None,
        }
    }
    #[doc = "vbus source over current"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VbusOvercurrentN::B0
    }
    #[doc = "vbus source not over current"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VbusOvercurrentN::B1
    }
}
#[doc = "Field `VBUS_OVERCURRENT_N` writer - vbus source overcurrent"]
pub type VbusOvercurrentNW<'a, REG> = crate::FieldWriter<'a, REG, 4, VbusOvercurrentN>;
impl<'a, REG> VbusOvercurrentNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "vbus source over current"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusOvercurrentN::B0)
    }
    #[doc = "vbus source not over current"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusOvercurrentN::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - TCPC vbus voltage\n\nTCPC vbus voltage"]
    #[inline(always)]
    pub fn vbus_voltage(&self) -> VbusVoltageR {
        VbusVoltageR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:13 - vbus source overcurrent"]
    #[inline(always)]
    pub fn vbus_overcurrent_n(&self) -> VbusOvercurrentNR {
        VbusOvercurrentNR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TCPC vbus voltage\n\nTCPC vbus voltage"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_voltage(&mut self) -> VbusVoltageW<Usb3phy1Con2Spec> {
        VbusVoltageW::new(self, 0)
    }
    #[doc = "Bits 10:13 - vbus source overcurrent"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_overcurrent_n(&mut self) -> VbusOvercurrentNW<Usb3phy1Con2Spec> {
        VbusOvercurrentNW::new(self, 10)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Usb3phy1Con2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3phy1_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3phy1_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3phy1Con2Spec;
impl crate::RegisterSpec for Usb3phy1Con2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3phy1_con2::R`](R) reader structure"]
impl crate::Readable for Usb3phy1Con2Spec {}
#[doc = "`write(|w| ..)` method takes [`usb3phy1_con2::W`](W) writer structure"]
impl crate::Writable for Usb3phy1Con2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3PHY1_CON2 to value 0x3cc8"]
impl crate::Resettable for Usb3phy1Con2Spec {
    const RESET_VALUE: u32 = 0x3cc8;
}
