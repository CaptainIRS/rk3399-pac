#[doc = "Register `GRF_USB20_PHY0_CON3` reader"]
pub type R = crate::R<GrfUsb20Phy0Con3Spec>;
#[doc = "Register `GRF_USB20_PHY0_CON3` writer"]
pub type W = crate::W<GrfUsb20Phy0Con3Spec>;
#[doc = "Field `IDPULLUP` reader - idpullup Use the value of this register input to idpullup of usb2phy"]
pub type IdpullupR = crate::BitReader;
#[doc = "Field `IDPULLUP` writer - idpullup Use the value of this register input to idpullup of usb2phy"]
pub type IdpullupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "drvvbus_sel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrvvbusSel {
    #[doc = "1: select drvvbus from usb3otg0 controller to drvvbus of usb2phy and GPIO to external PMIC"]
    B1 = 1,
    #[doc = "0: select drvvbus from usb3otg0 controller to drvvbus of usb2phy and GPIO to external PMIC"]
    B0 = 0,
}
impl From<DrvvbusSel> for bool {
    #[inline(always)]
    fn from(variant: DrvvbusSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRVVBUS_SEL` reader - drvvbus_sel"]
pub type DrvvbusSelR = crate::BitReader<DrvvbusSel>;
impl DrvvbusSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DrvvbusSel {
        match self.bits {
            true => DrvvbusSel::B1,
            false => DrvvbusSel::B0,
        }
    }
    #[doc = "select drvvbus from usb3otg0 controller to drvvbus of usb2phy and GPIO to external PMIC"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DrvvbusSel::B1
    }
    #[doc = "select drvvbus from usb3otg0 controller to drvvbus of usb2phy and GPIO to external PMIC"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DrvvbusSel::B0
    }
}
#[doc = "Field `DRVVBUS_SEL` writer - drvvbus_sel"]
pub type DrvvbusSelW<'a, REG> = crate::BitWriter<'a, REG, DrvvbusSel>;
impl<'a, REG> DrvvbusSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select drvvbus from usb3otg0 controller to drvvbus of usb2phy and GPIO to external PMIC"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DrvvbusSel::B1)
    }
    #[doc = "select drvvbus from usb3otg0 controller to drvvbus of usb2phy and GPIO to external PMIC"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DrvvbusSel::B0)
    }
}
#[doc = "Field `DRVVBUS` reader - drvvbus Pls see drvvbus_sel."]
pub type DrvvbusR = crate::BitReader;
#[doc = "Field `DRVVBUS` writer - drvvbus Pls see drvvbus_sel."]
pub type DrvvbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHRGVBUS` reader - chrgvbus Use the value of this register input to chrgvbus of usb2phy"]
pub type ChrgvbusR = crate::BitReader;
#[doc = "Field `CHRGVBUS` writer - chrgvbus Use the value of this register input to chrgvbus of usb2phy"]
pub type ChrgvbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCHRGVBUS` reader - dischrgvbus Use the value of this register input to chrgvbus of usb2phy"]
pub type DischrgvbusR = crate::BitReader;
#[doc = "Field `DISCHRGVBUS` writer - dischrgvbus Use the value of this register input to chrgvbus of usb2phy"]
pub type DischrgvbusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - idpullup Use the value of this register input to idpullup of usb2phy"]
    #[inline(always)]
    pub fn idpullup(&self) -> IdpullupR {
        IdpullupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - drvvbus_sel"]
    #[inline(always)]
    pub fn drvvbus_sel(&self) -> DrvvbusSelR {
        DrvvbusSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - drvvbus Pls see drvvbus_sel."]
    #[inline(always)]
    pub fn drvvbus(&self) -> DrvvbusR {
        DrvvbusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - chrgvbus Use the value of this register input to chrgvbus of usb2phy"]
    #[inline(always)]
    pub fn chrgvbus(&self) -> ChrgvbusR {
        ChrgvbusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - dischrgvbus Use the value of this register input to chrgvbus of usb2phy"]
    #[inline(always)]
    pub fn dischrgvbus(&self) -> DischrgvbusR {
        DischrgvbusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - idpullup Use the value of this register input to idpullup of usb2phy"]
    #[inline(always)]
    #[must_use]
    pub fn idpullup(&mut self) -> IdpullupW<GrfUsb20Phy0Con3Spec> {
        IdpullupW::new(self, 0)
    }
    #[doc = "Bit 1 - drvvbus_sel"]
    #[inline(always)]
    #[must_use]
    pub fn drvvbus_sel(&mut self) -> DrvvbusSelW<GrfUsb20Phy0Con3Spec> {
        DrvvbusSelW::new(self, 1)
    }
    #[doc = "Bit 2 - drvvbus Pls see drvvbus_sel."]
    #[inline(always)]
    #[must_use]
    pub fn drvvbus(&mut self) -> DrvvbusW<GrfUsb20Phy0Con3Spec> {
        DrvvbusW::new(self, 2)
    }
    #[doc = "Bit 3 - chrgvbus Use the value of this register input to chrgvbus of usb2phy"]
    #[inline(always)]
    #[must_use]
    pub fn chrgvbus(&mut self) -> ChrgvbusW<GrfUsb20Phy0Con3Spec> {
        ChrgvbusW::new(self, 3)
    }
    #[doc = "Bit 4 - dischrgvbus Use the value of this register input to chrgvbus of usb2phy"]
    #[inline(always)]
    #[must_use]
    pub fn dischrgvbus(&mut self) -> DischrgvbusW<GrfUsb20Phy0Con3Spec> {
        DischrgvbusW::new(self, 4)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsb20Phy0Con3Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "USB20 PHY0 GRF Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb20_phy0_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb20_phy0_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsb20Phy0Con3Spec;
impl crate::RegisterSpec for GrfUsb20Phy0Con3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usb20_phy0_con3::R`](R) reader structure"]
impl crate::Readable for GrfUsb20Phy0Con3Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usb20_phy0_con3::W`](W) writer structure"]
impl crate::Writable for GrfUsb20Phy0Con3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USB20_PHY0_CON3 to value 0x01"]
impl crate::Resettable for GrfUsb20Phy0Con3Spec {
    const RESET_VALUE: u32 = 0x01;
}
