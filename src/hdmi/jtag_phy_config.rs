#[doc = "Register `JTAG_PHY_CONFIG` reader"]
pub type R = crate::R<JtagPhyConfigSpec>;
#[doc = "Register `JTAG_PHY_CONFIG` writer"]
pub type W = crate::W<JtagPhyConfigSpec>;
#[doc = "Field `JTAG_TRST_N` reader - Configures the JTAG PHY interface output pin\n\nJTAG_TRST_N when in internal control mode\n\n(iphy_ext_ctrl=1'b0) or ophyext_jtag_trst_n when\n\nPHY_EXTERNAL=1."]
pub type JtagTrstNR = crate::BitReader;
#[doc = "Field `JTAG_TRST_N` writer - Configures the JTAG PHY interface output pin\n\nJTAG_TRST_N when in internal control mode\n\n(iphy_ext_ctrl=1'b0) or ophyext_jtag_trst_n when\n\nPHY_EXTERNAL=1."]
pub type JtagTrstNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Configures the JTAG PHY interface output pin\n\nI2C_JTAGZ to select the PHY configuration interface\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_i2c_jtagz when PHY_EXTERNAL=1.\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cJtagz {
    #[doc = "0: JTAG configuration Interface"]
    B0 = 0,
    #[doc = "1: I2C configuration Interface"]
    B1 = 1,
}
impl From<I2cJtagz> for bool {
    #[inline(always)]
    fn from(variant: I2cJtagz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_JTAGZ` reader - Configures the JTAG PHY interface output pin\n\nI2C_JTAGZ to select the PHY configuration interface\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_i2c_jtagz when PHY_EXTERNAL=1."]
pub type I2cJtagzR = crate::BitReader<I2cJtagz>;
impl I2cJtagzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cJtagz {
        match self.bits {
            false => I2cJtagz::B0,
            true => I2cJtagz::B1,
        }
    }
    #[doc = "JTAG configuration Interface"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == I2cJtagz::B0
    }
    #[doc = "I2C configuration Interface"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == I2cJtagz::B1
    }
}
#[doc = "Field `I2C_JTAGZ` writer - Configures the JTAG PHY interface output pin\n\nI2C_JTAGZ to select the PHY configuration interface\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_i2c_jtagz when PHY_EXTERNAL=1."]
pub type I2cJtagzW<'a, REG> = crate::BitWriter<'a, REG, I2cJtagz>;
impl<'a, REG> I2cJtagzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "JTAG configuration Interface"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(I2cJtagz::B0)
    }
    #[doc = "I2C configuration Interface"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(I2cJtagz::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Configures the JTAG PHY interface output pin\n\nJTAG_TRST_N when in internal control mode\n\n(iphy_ext_ctrl=1'b0) or ophyext_jtag_trst_n when\n\nPHY_EXTERNAL=1."]
    #[inline(always)]
    pub fn jtag_trst_n(&self) -> JtagTrstNR {
        JtagTrstNR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Configures the JTAG PHY interface output pin\n\nI2C_JTAGZ to select the PHY configuration interface\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_i2c_jtagz when PHY_EXTERNAL=1."]
    #[inline(always)]
    pub fn i2c_jtagz(&self) -> I2cJtagzR {
        I2cJtagzR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the JTAG PHY interface output pin\n\nJTAG_TRST_N when in internal control mode\n\n(iphy_ext_ctrl=1'b0) or ophyext_jtag_trst_n when\n\nPHY_EXTERNAL=1."]
    #[inline(always)]
    #[must_use]
    pub fn jtag_trst_n(&mut self) -> JtagTrstNW<JtagPhyConfigSpec> {
        JtagTrstNW::new(self, 0)
    }
    #[doc = "Bit 4 - Configures the JTAG PHY interface output pin\n\nI2C_JTAGZ to select the PHY configuration interface\n\nwhen in internal control mode (iphy_ext_ctrl=1'b0)\n\nor ophyext_jtag_i2c_jtagz when PHY_EXTERNAL=1."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_jtagz(&mut self) -> I2cJtagzW<JtagPhyConfigSpec> {
        I2cJtagzW::new(self, 4)
    }
}
#[doc = "PHY I2C/JTAG I/O Configuration Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtag_phy_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_phy_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JtagPhyConfigSpec;
impl crate::RegisterSpec for JtagPhyConfigSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`jtag_phy_config::R`](R) reader structure"]
impl crate::Readable for JtagPhyConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`jtag_phy_config::W`](W) writer structure"]
impl crate::Writable for JtagPhyConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets JTAG_PHY_CONFIG to value 0x11"]
impl crate::Resettable for JtagPhyConfigSpec {
    const RESET_VALUE: u8 = 0x11;
}
