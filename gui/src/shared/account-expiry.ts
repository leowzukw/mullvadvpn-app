import moment from 'moment';
import { sprintf } from 'sprintf-js';
import { messages } from './gettext';

export class AccountExpiry {
  protected expiry: moment.Moment;

  constructor(isoString: string) {
    this.expiry = moment(isoString);
  }

  public get moment() {
    return this.expiry;
  }

  public hasExpired(): boolean {
    return this.willHaveExpiredAt(new Date());
  }

  public willHaveExpiredInThreeDays(): boolean {
    return this.willHaveExpiredAt(moment().add(3, 'days').toDate());
  }

  public willHaveExpiredAt(date: Date): boolean {
    return this.expiry.isSameOrBefore(date);
  }

  public remainingMilliseconds(): number {
    return this.expiry.diff(new Date());
  }
}

export class AccountExpiryFormatter extends AccountExpiry {
  constructor(isoString: string, locale: string) {
    super(isoString);
    this.expiry = this.expiry.locale(locale);
  }

  public formattedDate(): string {
    return this.expiry.format('lll');
  }

  public durationUntilExpiry(): string {
    const daysDiff = this.expiry.diff(new Date(), 'days');

    // Below three months we want to show the duration in days. Moments fromNow() method starts
    // measuring duration in months from 26 days and up.
    // https://momentjs.com/docs/#/displaying/fromnow/
    if (daysDiff >= 26 && daysDiff <= 90) {
      return sprintf(
        // TRANSLATORS: The remaining time left on the account measured in days.
        // TRANSLATORS: Available placeholders:
        // TRANSLATORS: %(duration)s - The remaining time measured in days.
        messages.pgettext('account-expiry', '%(duration)s days'),
        { duration: daysDiff },
      );
    } else {
      return this.expiry.fromNow(true);
    }
  }

  public remainingTime(shouldCapitalizeFirstLetter?: boolean): string {
    const duration = this.durationUntilExpiry();

    const remaining = sprintf(
      // TRANSLATORS: The remaining time left on the account displayed across the app.
      // TRANSLATORS: Available placeholders:
      // TRANSLATORS: %(duration)s - a localized remaining time (in minutes, hours, or days) until the account expiry
      messages.pgettext('account-expiry', '%(duration)s left'),
      { duration },
    );

    return shouldCapitalizeFirstLetter ? capitalizeFirstLetter(remaining) : remaining;
  }
}

function capitalizeFirstLetter(inputString: string): string {
  return inputString.charAt(0).toUpperCase() + inputString.slice(1);
}
